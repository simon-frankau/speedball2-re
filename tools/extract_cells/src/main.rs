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
            data: vec![192; width * height * 3],
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
            let pixel = pixel_iter.next().unwrap();
            img.set_pixel(x + x_off, y + y_off, pixel);
        }
    }
}

// Draw out a set of cells, stored sequentially.
fn draw_cells(img: &mut Image, x: usize, y: usize, data: &[u8], w: usize, h: usize) {
    let mut cell_data_iter = data.chunks(CELL_LEN);

    for cy in 0..h {
        for cx in 0..w {
            let next_cell_data = cell_data_iter.next().unwrap();
            draw_cell(img, x + cx * CELL_SIZE, y + cy * CELL_SIZE, &next_cell_data);
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

// Width and height in cells.
fn build_image(data: &[u8], w: usize, h: usize, palette: &[u8]) -> Image {
    let mut img = Image::new(w * CELL_SIZE, h * CELL_SIZE, build_palette(palette));
    draw_cells(&mut img, 0, 0, data, w, h);

    img
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    let img_data = &data[10..0x0454ca];
    let total_cells = img_data.len() / CELL_LEN;
    let w = 12; // Seems a reasonable width.

    for (idx, palette_addr) in PALETTE_ADDRS.iter().enumerate() {
        println!("Run #{}", idx);
        let palette = &data[*palette_addr..];
        let img = build_image(img_data, w, total_cells / w, palette);
        img.save(Path::new(format!("cells-colour-{:02}.png", idx).as_str()));
    }
}
