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

// Draw sw x sh block of cells at (x, y)
fn draw_sprite(img: &mut Image, x: usize, y: usize, data: &[u8], sw: usize, sh: usize) {
    for sy in 0..sh {
        for sx in 0..sw {
            draw_cell(
                img,
                x + CELL_SIZE * sx,
                y + CELL_SIZE * sy,
                &data[CELL_LEN * (sy * sw + sx)..]);
        }
    }
}

// Draw out a bunch of sprites
fn draw_sprites(img: &mut Image, data: &[u8], sw: usize, sh: usize) {
    // We assume all the sprites fit in img, and img's width is a multiple
    let (mut x, mut y) = (0, 0);

    for block in data.chunks_exact(CELL_LEN * sw * sh) {
        draw_sprite(img, x, y, &block, sw, sh);
        x += sw * CELL_SIZE;
        if x >= img.width {
            x = 0;
            y += sh * CELL_SIZE;
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
fn build_image(img_data: &[u8], w: usize, palette_data: &[u8], sw: usize, sh: usize) -> Image {
    let num_sprites = img_data.len() / (CELL_LEN * sw * sh);

    let h = (num_sprites - 1) / w + 1;

    let mut img = Image::new(
        w * sw * CELL_SIZE,
        h * sh * CELL_SIZE,
        build_palette(palette_data));
    draw_sprites(&mut img, img_data, sw, sh);
    img
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    for (idx, palette) in PALETTE_ADDRS.iter().enumerate() {
        println!("Run #{}", idx);
        for (start, end, width, height, name) in &[
            (0x0610c4, 0x068244, 2, 2, "2x2"),
            (0x068244, 0x072444, 4, 4, "4x4"),
            (0x072444, 0x074284, 1, 1, "1x1"),
            (0x074284, 0x07e004, 2, 2, "players"),
        ] {
            let w = 36 / width;
            let img_data = &data[*start..*end];
            let palette_data = &data[*palette..];
            let img = build_image(img_data, w, palette_data, *width, *height);
            img.save(Path::new(format!("cells-{}-{:02}.png", name, idx).as_str()));
        }
    }
}
