//
// Palette finder
//
// Find the addresses that may contain a palette.
//

use std::fs;

use image::RgbImage;

// 16 colour palette.
const PALETTE_SIZE: usize = 16;
// 2 bytes per entry.
const PALETTE_LEN: usize = PALETTE_SIZE * 2;

// Assume real palettes don't have repeated colours.
const NO_REPEATS: bool = true;

// Assume real palettes start with black.
const START_WITH_BLACK: bool = true;

// Does the data at the start of the slice look like a potential
// palette entry?
fn looks_like_colour(data: &[u8]) -> bool {
    let (msb, lsb) = (data[0], data[1]);

    ((msb & 0xf1) == 0) && ((lsb & 0x11) == 0)
}

// Looks like a palette if we have 16 consecutive colours.
fn looks_like_palette(data: &[u8]) -> bool {
   if START_WITH_BLACK && (data[0] != 0 || data[1] != 0) {
       return false;
   }

    for i in 0..16 {
        if !looks_like_colour(&data[2 * i..]) {
            return false;
        }
    }

    // All zero probably isn't a palette
    if !data[..16].iter().any(|x| *x != 0) {
       return false;
    }

    if NO_REPEATS {
        // Check for repeats, inefficiently (n^2, small n)
        for i in 0 .. PALETTE_SIZE {
            for j in i + 1 .. PALETTE_SIZE {
                if data[i*2] == data[j*2] && data[i*2+1] == data[j*2+1] {
                    return false;
                }
            }
        }
    }

    true
}

fn extract_colour(data: &[u8]) -> Vec<u8> {
    // 3-bit RGB values.
    let r = (data[1] >> 1) & 7;
    let g = (data[1] >> 5) & 7;
    let b = (data[0] >> 1) & 7;

    vec![r << 5, g << 5, b << 5]
}

fn palette_to_image_row(palette: &[u8]) -> Vec<u8> {
    palette
        .chunks_exact(2)
        .flat_map(|v| extract_colour(v))
        .collect()
}

fn palettes_to_image(palettes: &[&[u8]]) -> RgbImage {
    let width = palettes.iter().map(|v| v.len()).max().unwrap();
    let height = palettes.len();

    let image_data = palettes
        .iter()
        .flat_map(|v| {
            let mut row = palette_to_image_row(v);
            row.resize(width * 3, 255);
            row
        })
        .collect();

    RgbImage::from_vec(width as u32, height as u32, image_data).unwrap()
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    // Find all the potential palettes, printing their locations and
    // stashing the palettes.
    let mut palettes: Vec<&[u8]> = Vec::new();

    // Palettes tend to be odd and even, so this logic is messy...
    let mut last_maybe_palette = false;
    let mut last_maybe_palette2 = false;
    let mut start = 0;
    for (idx, window) in data.windows(PALETTE_LEN).enumerate() {
        let maybe_palette = looks_like_palette(window);
        if maybe_palette && !last_maybe_palette && !last_maybe_palette2 {
            print!("{:06x}..", idx);
            start = idx;
        } else if !maybe_palette && !last_maybe_palette && last_maybe_palette2 {
            println!("{:06x} ({})", idx, idx - start);
            palettes.push(&data[start..idx - 2 + PALETTE_LEN]);
        }
        last_maybe_palette2 = last_maybe_palette;
        last_maybe_palette = maybe_palette;
    }

    // And now, make a lovely little picture of the possible palettes.
    let image = palettes_to_image(&palettes);
    image.save("palettes.png").unwrap();
}
