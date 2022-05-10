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
use std::str;

use image::RgbImage;

// A cell is 8x8.
const CELL_SIZE: usize = 8;

// Each byte stores 2 pixels.
const CELL_LEN: usize = CELL_SIZE * CELL_SIZE / 2;

// 16 colour palette.
const PALETTE_SIZE: usize = 16;
// 2 bytes per entry.
const PALETTE_LEN: usize = PALETTE_SIZE * 2;

// Screen width and height, in cells
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 25;
// 2 bytes per cell reference.
const SCREEN_LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 2;

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

fn draw_cell(img: &mut Image, x: usize, y: usize, data: &[u8], h_flip: bool, v_flip:bool) {
    // Inefficient, but can't iterate an array, or return an iterator over it.
    let mut pixel_iter = data.iter().flat_map(|p| vec![p >> 4, p & 0xf]);

    for y_off in 0..CELL_SIZE {
        for x_off in 0..CELL_SIZE {
            let pixel = pixel_iter.next().unwrap();
            let adj_y_off = if v_flip { CELL_SIZE - 1 - y_off } else { y_off };
            let adj_x_off = if h_flip { CELL_SIZE - 1 - x_off } else { x_off };
            img.set_pixel(x + adj_x_off, y + adj_y_off, pixel);
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

fn decompress(data: &[u8]) -> Vec<u8> {
    fn as_u32(data: &[u8], idx: usize) -> u32 {
        (data[idx] as u32) << 24 | (data[idx+1] as u32) << 16 |
            (data[idx+2] as u32) << 8 | data[idx+3] as u32
    }

    assert_eq!(str::from_utf8(&data[..16]).unwrap(), "QPAC2-JMP(C)1989");

    let len = as_u32(data, 16) as usize;
    let checksum = as_u32(data, 20);
    let mut ptr = 24; // ptr always points to offset of next unread data.
    let mut curr = 0; // Last data to be loaded.

    fn next_bit(ptr: &mut usize, curr: &mut u32, data: &[u8]) -> bool {
        if *curr << 1 == 0 {
            *curr = as_u32(data, *ptr);
            *ptr += 4;
            let res = *curr & 0x80000000 != 0;
            *curr = *curr << 1 | 1;
            res
        } else {
            let res = *curr & 0x80000000 != 0;
            *curr = *curr << 1;
            res
        }
    };

    fn next_int(ptr: &mut usize, curr: &mut u32, data: &[u8], i: usize) -> usize {
        let mut x: usize = 0;
        for _ in 0..i {
            x = (x << 1) | if next_bit(ptr, curr, data) { 1 } else { 0 };
        }
        x
    };

    println!("Decompressing. Uncompressed length: {:08x} Checksum: {:08x}",
        len, checksum);

    let mut res: Vec<u8> = Vec::new();

    while res.len() < len {
       if next_bit(&mut ptr, &mut curr, data) {
           if next_bit(&mut ptr, &mut curr, data) {
               if next_bit(&mut ptr, &mut curr, data) {
                   // 111 XXXX XXXX ...
                   // Copy x + 9 bytes to output stream.
                   let c = next_int(&mut ptr, &mut curr, data, 8) + 9;
                   // println!("111 copy {}", c);
                   for _ in 0..c {
                       res.push(next_int(&mut ptr, &mut curr, data, 8) as u8);
                   }
               } else {
                   // 110 XXXX XXXX YYYY YYYY YYYY
                   // Look back Y + 1 bytes, copy X + 3 bytes.
                   let c = next_int(&mut ptr, &mut curr, data, 8) + 3;
                   let lb = next_int(&mut ptr, &mut curr, data, 12) + 1;
                   // println!("110 lb {} c {} @ {}", lb, c, res.len());
                   let mut src = res.len() - lb;
                   for _ in 0..c {
                       res.push(res[src]);
                       src += 1;
                   }
               }
           } else {
               if next_bit(&mut ptr, &mut curr, data) {
                   // 101 XXXX XXXX XX
                   // Look back X + 1 bytes, copy 4 bytes of data.
                   let lb = next_int(&mut ptr, &mut curr, data, 10) + 1;
                   // println!("101 lb {} c 4 @ {}", lb, res.len());
                   let mut src = res.len() - lb;
                   for _ in 0..4 {
                       res.push(res[src]);
                       src += 1;
                   }
               } else {
                   // 100 XXXX XXXX X
                   // Look back X + 1 bytes, copy 3 bytes of data.
                   let lb = next_int(&mut ptr, &mut curr, data, 9) + 1;
                   // println!("100 lb {} c 3 @ {}", lb, res.len());
                   let mut src = res.len() - lb;
                   for _ in 0..3 {
                       res.push(res[src]);
                       src += 1;
                   }
               }
           }
       } else {
           if next_bit(&mut ptr, &mut curr, data) {
               // 01 XXXX XXXX
               // Look back X + 1 bytes, copy 2 bytes of data.
               let lb = next_int(&mut ptr, &mut curr, data, 8) + 1;
               // println!("01 lb {} c 2 @ {}", lb, res.len());
               let mut src = res.len() - lb;
               for _ in 0..2 {
                   res.push(res[src]);
                   src += 1;
               }
           } else {
               // 00 XXX ....
               // Copy X + 1 bytes of data directly to the output stream.
               let c = next_int(&mut ptr, &mut curr, data, 3) + 1;
               // println!("00 copy {}", c);
               for _ in 0..c {
                   res.push(next_int(&mut ptr, &mut curr, data, 8) as u8);
               }
           }
       }
    }

    res
}

fn draw_splash(data: &[u8], mut addr: usize, file_name: &Path) {
    // First element: type.
    let splash_type = (data[addr] as u16) << 8 | data[addr + 1] as u16;
    addr += 2;

    // Second element: palette.
    let pal = build_palette(&data[addr..addr+PALETTE_LEN]);
    addr += PALETTE_LEN;

    // Remaining data may be compressed.
    let remaining_data = match splash_type {
        0 => data[addr..].to_vec(),
        1 => decompress(&data[addr..]),
        _ => panic!("Unrecognised type: {:04x}", splash_type),
    };

    let tile_map = &remaining_data[..SCREEN_LEN];
    let cells = &remaining_data[SCREEN_LEN..];

    let mut img = Image::new(SCREEN_WIDTH * CELL_SIZE, SCREEN_HEIGHT * CELL_SIZE, pal);

    let mut map_idx = 0;
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let tile_data = (tile_map[map_idx] as u16) << 8 | tile_map[map_idx + 1] as u16;
            map_idx += 2;
            let tile_num = tile_data & 0x07ff;
            let h_flip = tile_data & 0x0800 != 0;
            let v_flip = tile_data & 0x1000 != 0;
            if tile_data & 0xe000 != 0 {
                println!("Warning at {}, {}: {:04x}", x, y, tile_data - tile_num);
            }

            let tile_addr = tile_num as usize * CELL_LEN;
            draw_cell(&mut img, x * CELL_SIZE, y * CELL_SIZE, &cells[tile_addr..], h_flip, v_flip);
        }
    }

    img.save(file_name);
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();
    for (addr, name) in SCREENS.iter() {
        println!("Processing {} ({:08x})...", name, *addr);
        draw_splash(&data, *addr, &Path::new(name));
    }
}
