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

// Palettes found in the file by `find_palettes`, and some detective
// work.
const PALETTE_ADDRS: [(usize, &str); 23] = [
    (0x0007c4, "palette_game_a"),
    (0x0007e4, "palette_game_b"),
    (0x000804, "palette_game_c"),
    (0x02260c, "splash_start1"),
    (0x025a5e, "splash_start2"),
    (0x029e5e, "palette_gold_a"),
    (0x029e7e, "palette_gold_b"),
    (0x029e9e, "palette_gold_c"),
    (0x029ebe, "palette_mono"),
    (0x029ede, "palette_training_a"),
    (0x029efe, "palette_training_b"),
    (0x029f1e, "palette_magenta_a"),
    (0x029f3e, "palette_magenta_b"),
    (0x029f5e, "palette_backdrop_a"),
    (0x029f7e, "palette_backdrop_b"),
    (0x0454cc, "splash_backdrop"),
    (0x049bfe, "splash_victory"),
    (0x051fe2, "splash_win_league"),
    (0x053e1c, "splash_win_promo"),
    (0x055c36, "splash_win_cup"),
    (0x057a54, "splash_win_knockout"),
    (0x05983a, "splash_title"),
    (0x05d8cc, "splash_arena"),
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
            data: vec![0xc0; width * height * 3],
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
fn draw_sprite(img: &mut Image, x: usize, y: usize, data: &[u8], sw: usize, sh: usize, transpose: bool) {
    for sy in 0..sh {
        for sx in 0..sw {
            draw_cell(
                img,
                x + CELL_SIZE * if transpose { sy } else { sx },
                y + CELL_SIZE * if transpose { sx } else { sy },
                &data[CELL_LEN * (sy * sw + sx)..]);
        }
    }
}

// Draw out a bunch of sprites
fn draw_sprites(img: &mut Image, data: &[u8], sw: usize, sh: usize,
    transpose: bool) {
    // We assume all the sprites fit in img, and img's width is a multiple
    let (mut x, mut y) = (0, 0);

    for block in data.chunks_exact(CELL_LEN * sw * sh) {
        draw_sprite(img, x, y, &block, sw, sh, transpose);
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
fn build_image(img_data: &[u8], w: usize, palette_data: &[u8], sw: usize, sh: usize, transpose: bool) -> Image {
    let num_sprites = img_data.len() / (CELL_LEN * sw * sh);

    let h = (num_sprites - 1) / w + 1;

    let mut img = Image::new(
        w * sw * CELL_SIZE,
        h * sh * CELL_SIZE,
        build_palette(palette_data));
    draw_sprites(&mut img, img_data, sw, sh, transpose);
    img
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    for (palette, palette_name) in PALETTE_ADDRS.iter() {
        println!("Run for palette '{}'", palette_name);
        for (start, end, width, height, transpose, name) in &[
            (0x0291da, 0x297fa, 1, 1, false, "sega_logo"),
            (0x029fde, 0x02a0de, 1, 1, false, "push_start"),
	    // TODO: Backdrop palette seems right?
            (0x02d7ec, 0x02e0ec, 1, 1, false, "passwords_font_1x1"),
            // TODO: Uses palette from splash_arena.
            (0x02e6ca, 0x02fcaa,  2, 2, true,  "title_font_2x2t"),
	    // Game palette...
            (0x02fd0a, 0x03070a,  2, 2, true,  "game_status_bar_2x2t"),
            (0x03070a, 0x03084a,  1, 1, false, "game_score_digits_1x1"),
            (0x03084a, 0x032c4a, 12, 8, false, "game_monitor_12x8"),
            (0x032c4a, 0x0330ca,  1, 1, false, "game_font_1x1"),
            (0x0330ca, 0x034c4a,  2, 2, true,  "game_misc_2x2t"),
            (0x034c4a, 0x035dca,  2, 2, false, "game_tokens_2x2"),
            (0x035dca, 0x03efca,  4, 4, true,  "game_players_4x4t"),
            (0x03efca, 0x0403ca,  4, 4, true,  "game_medibot_4x4t"),
            (0x0403ca, 0x0425ca,  4, 4, true,  "game_ball_stuff_4x4t"),
            (0x0425ca, 0x04286a,  1, 1, false, "game_arena_1x1"),
            (0x04286a, 0x0454ca,  4, 4, false, "game_arena_4x4"),

            // second set of sprites, near the end of the ROM.

	    // TODO: Backdrop palette.
            (0x0610c4, 0x0623c4, 2, 2, false, "menu_font"),
	    // TODO: Training palette.
            (0x0623c4, 0x067a44, 2, 2, false, "training_background_2x2"),
            (0x067a44, 0x068244, 2, 2, false, "training_lights_2x2"),
            (0x068244, 0x06da44, 4, 4, false, "training_buttons_4x4"),
            (0x06da44, 0x072444, 4, 4, false, "training_armour_4x4"),

	    // TODO: Backdrop palette
            (0x072444, 0x072aa4, 1, 1, false, "font_orange_1x1"),
            (0x072aa4, 0x072e44, 1, 1, false, "font_title_top_1x1"),
            (0x072e44, 0x073284, 1, 1, false, "font_title_bottom_1x1"),
	    // TODO: Training palette.
	    (0x073284, 0x0734e4, 1, 1, false, "font_mgr_xfer_gym_1x1"),
            (0x0734e4, 0x073964, 1, 1, false, "font_cash_1x1"),
	    (0x073964, 0x073e04, 1, 1, false, "font_small_green_1x1"),
	    // TODO: Backdrop palette
	    (0x073e04, 0x074284, 1, 1, false, "font_white_1x1"),

	    // TODO: Training palette.
            (0x074284, 0x07e004, 2, 2, false, "training_faces_6x6"),
        ] {
            let w = 36 / width;
            let img_data = &data[*start..*end];
            let palette_data = &data[*palette..];
            let img = build_image(img_data, w, palette_data, *width, *height, *transpose);
            img.save(Path::new(format!("{}-{}.png", name, palette_name).as_str()));
        }
    }
}
