//
// Screen displayer
//
//
// Pulls a static screen structure from the Speedball 2 ROM and
// converts it into an image file. Understands the specific structure
// used, with a palette, tile map, and set of cells.
//

use std::fs;
use std::path::Path;

use image::RgbImage;

// A cell is 8x8.
const CELL_SIZE: usize = 8;

// Each byte stores 2 pixels.
const CELL_LEN: usize = CELL_SIZE * CELL_SIZE / 2;

// 16 colour palette.
const PALETTE_SIZE: usize = 16;

// Screen width and height, in cells
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 25;

// Locations of screens passed to display_splash.
const SCREENS: [(usize, &str); 11] = [
    (0x02260a, "start1.png"),
    (0x025a5c, "start2.png"),
    (0x0454ca, "backdrop.png"),
    (0x049bfc, "victory.png"),
    (0x04e66e, "defeat.png"),
    (0x051fe0, "unknown2.png"),
    (0x053e1a, "unknown3.png"),
    (0x055c34, "unknown4.png"),
    (0x057a52, "unknown5.png"),
    (0x059838, "unknown7.png"),
    (0x05d8ca, "unknown8.png"),
];

////////////////////////////////////////////////////////////////////////
// Cheap wrapper around the image we're producing.
//

struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    palette: Vec<(u8, u8, u8)>,
}

impl Image {
    fn new(width: usize, height: usize, palette: Vec<(u8, u8, u8)>) -> Image {
        Image {
            width,
            height,
            data: vec![0; width * height * 3],
            palette,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        let idx = (y * self.width + x) * 3;
        let (r, g, b) = self.palette[value as usize];
        self.data[idx] = r;
        self.data[idx + 1] = g;
        self.data[idx + 2] = b;
    }

    fn save(&self, path: &Path) {
        let img =
            RgbImage::from_vec(self.width as u32, self.height as u32, self.data.clone()).unwrap();
        img.save(path).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////
// Functions to draw raw binary cells to an image
//

fn draw_cell(img: &mut Image, x: usize, y: usize, data: &[u8]) {
    // Inefficient, but can't iterate an array, or return an iterator over it.
    let mut pixel_iter = data.iter().flat_map(|p| vec![p >> 4, p & 0xf]);

    for y_off in 0..CELL_SIZE {
        for x_off in 0..CELL_SIZE {
            let pixel = pixel_iter.next().unwrap();
            img.set_pixel(x + x_off, y + y_off, pixel);
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Palette stuff
//

fn extract_colour(data: &[u8]) -> (u8, u8, u8) {
    // 3-bit RGB values.
    let r = (data[1] >> 1) & 7;
    let g = (data[1] >> 5) & 7;
    let b = (data[0] >> 1) & 7;

    (r << 5, g << 5, b << 5)
}

// Build a palette from a piece of memory.
fn build_palette(data: &[u8]) -> Vec<(u8, u8, u8)> {
    data.chunks(2).take(PALETTE_SIZE).map(|v| extract_colour(v)).collect()
}

////////////////////////////////////////////////////////////////////////
// Main algorithm
//

fn draw_splash(data: &[u8], mut addr: usize, file_name: &Path) {
    // Extract the addresses of the elements.
    // Skip initial word.
    println!("Mystery word: {}{}", data[addr], data[addr+1]);
    addr += 2;
    let pal_addr = addr;
    addr += PALETTE_SIZE * 2; // 2 bytes per entry.
    let map_addr = addr;
    addr += SCREEN_WIDTH * SCREEN_HEIGHT * 2; // Ditto.
    let cell_addr = addr;


    let pal = build_palette(&data[pal_addr..]);
    let mut img = Image::new(SCREEN_WIDTH * CELL_SIZE, SCREEN_HEIGHT * CELL_SIZE, pal);

    let mut map_ptr = map_addr;
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let tile_data = (data[map_ptr] as u16) << 8 | data[map_ptr + 1] as u16;
            map_ptr += 2;
            let tile_num = tile_data & 0x7ff;
            if tile_num != tile_data {
                println!("Warning at {}, {}: {:04x}", x, y, tile_data - tile_num);
            }

            let tile_addr = cell_addr + tile_num as usize * CELL_LEN;
            draw_cell(&mut img, x * CELL_SIZE, y * CELL_SIZE, &data[tile_addr..]);
        }
    }

    img.save(file_name);
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();
    for (addr, name) in SCREENS.iter() {
        println!("Processing {}...", name);
        draw_splash(&data, *addr, &Path::new(name));
    }
}