//
// Cell extractor
//
// Breaks down a binary file into Megadrive-style 8x8 image 'cells',
// and constructs an image from that. Use to find bitmaps in a
// Megadrive ROM.
//

use std::fs;
use std::path::Path;

use image::GrayImage;

// A cell is 8x8.
const CELL_SIZE: usize = 8;

// Each byte stores 2 pixels.
const CELL_LEN: usize = CELL_SIZE * CELL_SIZE / 2;

////////////////////////////////////////////////////////////////////////
// Cheap wrapper around the image we're producing.
//

struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.data[y * self.width + x] = value;
    }

    fn save(&self, path: &Path) {
        let img =
            GrayImage::from_vec(self.width as u32, self.height as u32, self.data.clone()).unwrap();
        img.save(path).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////
// Functions to draw raw binary cells to an image
//

// One cell
fn draw_cell(img: &mut Image, x: usize, y:usize, data: &[u8]) {
    // Inefficient, but can't iterate an array, or return an iterator over it.
    let mut pixel_iter = data.iter().flat_map(|p| vec![p >> 4, p & 0xf]);

    for x_off in 0..CELL_SIZE {
        for y_off in 0..CELL_SIZE {
            let raw_pixel = pixel_iter.next().unwrap();
            let pixel = raw_pixel * 16; // TODO: Palette lookup.
            img.set_pixel(x + x_off, y + y_off, pixel);
        }
    }
}

// Draw out a set of cells, stored sequentially.
fn draw_cells(img: &mut Image, x: usize, y:usize, data: &[u8], w: usize, h:usize) {
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

fn main() {
    let data = fs::read("../speedball2-usa.bin").unwrap();

    let mut my_image = Image::new(64, 64);
    for i in 16..48 {
        my_image.set_pixel(i, i + 10, 128 + i as u8);
    }

    draw_cells(&mut my_image, 32, 32, &data[100000..], 3, 2);

    my_image.save(Path::new("my_image.png"));
}
