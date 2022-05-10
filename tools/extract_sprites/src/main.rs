//
// Cell extractor
//
// Breaks down a binary file into Megadrive-style 8x8 image 'cells',
// and constructs an image from that. Use to find bitmaps in a
// Megadrive ROM.
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

// Palettes found in the file by `find_palettes`.
const PALETTE_ADDRS: [usize; 13] = [
    0x0007c4, 0x02260c, 0x025a5e, 0x029ede, 0x029efe, 0x0454cc, 0x049bfe, 0x051fe2, 0x053e1c,
    0x055c36, 0x057a54, 0x05983a, 0x05d8cc,
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

// One cell
fn draw_cell(img: &mut Image, x: usize, y: usize, data: &[u8]) {
    // Inefficient, but can't iterate an array, or return an iterator over it.
    let mut pixel_iter = data.iter().flat_map(|p| vec![p >> 4, p & 0xf]);

    for y_off in 0..CELL_SIZE {
        for x_off in 0..CELL_SIZE {
            let pixel = pixel_iter.next();
            if pixel == None {
                break;
            }
            img.set_pixel(x + x_off, y + y_off, pixel.unwrap());
        }
    }
}

// 2x2 block of cells
fn draw_cells_2x2(img: &mut Image, x: usize, y: usize, data: &[u8]) {
    draw_cell(img, x, y, data);
    draw_cell(img, x + CELL_SIZE, y, &data[CELL_LEN..]);
    draw_cell(img, x, y + CELL_SIZE, &data[CELL_LEN * 2..]);
    draw_cell(img, x + CELL_SIZE, y + CELL_SIZE, &data[CELL_LEN * 3..]);
}

// Draw out a bunch of 2x2 blocks.
fn draw_cells_2x2_multi(img: &mut Image, data: &[u8]) {
    // We assume all the sprites fit in img, and img's width is a multiple
    let (mut x, mut y) = (0, 0);

    for block in data.chunks_exact(CELL_LEN * 4) {
        draw_cells_2x2(img, x, y, &block);
        x += 2 * CELL_SIZE;
        if x >= img.width {
            x = 0;
            y += 2 * CELL_SIZE;
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Entry point
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

// Width in sprites.
fn build_image(data: &[u8], w: usize, palette_addr: usize) -> Image {
    let img_data = &data[0x0610c4..];
    let num_sprites = img_data.len() / (CELL_LEN * 4);

    let h = (num_sprites - 1) / w + 1;

    let mut img = Image::new(
        w * 2 * CELL_SIZE,
        h * 2 * CELL_SIZE,
        build_palette(&data[palette_addr..]));
    draw_cells_2x2_multi(&mut img, img_data);
    img
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    let w = 16;

    for (idx, palette) in PALETTE_ADDRS.iter().enumerate() {
        println!("Run #{}", idx);
        let img = build_image(&data, w, *palette);
        img.save(Path::new(format!("cells-colour-{:02}.png", idx).as_str()));
    }
}
