//
// Palette finder
//
// Find the addresses that may contain a palette.
//

use std::fs;

// 16 colour palette.
const PALETTE_SIZE: usize = 16;
// 2 bytes per entry.
const PALETTE_LEN: usize = PALETTE_SIZE * 2;

// Does the data at the start of the slice look like a potential
// palette entry?
fn looks_like_colour(data: &[u8]) -> bool {
    let (msb, lsb) = (data[0], data[1]);

    ((msb & 0xf1) == 0) && ((lsb & 0x11) == 0)
}

// Looks like a palette if we have 16 consecutive colours.
fn looks_like_palette(data: &[u8]) -> bool {
    for i in 0..16 {
        if !looks_like_colour(&data[2 * i ..]) {
            return false;
        }
    }

    // All zero probably isn't a palette
    data[..16].iter().any(|x| *x != 0)
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

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
        }
        last_maybe_palette2 = last_maybe_palette;
        last_maybe_palette = maybe_palette;
    }
}
