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

// Location of the various palettes we use.
const SEGA: usize = 0x02914e;
const GAME_RED: usize = 0x0007c4;
const GAME_BLUE: usize = 0x0007e4;
const GAME: usize = 0x000804;
const NON_GAME: usize = 0x029f5e;
const MGMT: usize = 0x029efe;
const ARENA: usize = 0x05d8cc;

// Output directory
const OUT_DIR: &str = "out";

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

    fn save(&self, path: &Path) -> anyhow::Result<()> {
        let img =
            RgbImage::from_vec(self.width as u32, self.height as u32, self.data.clone()).unwrap();
        img.save(path)?;
	Ok(())
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

fn main() -> anyhow::Result<()> {
    let data = fs::read("../../speedball2-usa.bin")?;

    fs::create_dir_all(OUT_DIR)?;
    
    for (start, end, width, height, transpose, palette, name) in &[
        (0x0291da, 0x0297fa,  1, 1, false, SEGA,      "sega_logo"),
	// I think this is black & white when displayed outside a match.
        (0x029fde, 0x02a0de,  1, 1, false, GAME,      "push_start"),
        (0x02d7ec, 0x02e0ec,  1, 1, false, NON_GAME,  "passwords_font_1x1"),
        (0x02e6ca, 0x02fcaa,  2, 2, true,  ARENA,     "title_font_2x2t"),
        (0x02fd0a, 0x03070a,  2, 2, true,  GAME,      "game_status_bar_2x2t"),
        (0x03070a, 0x03084a,  1, 1, false, GAME,      "game_score_digits_1x1"),
        (0x03084a, 0x032c4a, 12, 8, false, GAME,      "game_monitor_12x8"),
        (0x032c4a, 0x0330ca,  1, 1, false, GAME,      "game_font_1x1"),
        (0x0330ca, 0x034c4a,  2, 2, true,  GAME,      "game_misc_2x2t"),
        (0x034c4a, 0x035dca,  2, 2, false, GAME,      "game_tokens_2x2"),
	(0x035dca, 0x03efca,  4, 4, true,  GAME_RED,  "game_players_red_4x4t"),
	(0x035dca, 0x03efca,  4, 4, true,  GAME_BLUE, "game_players_blue_4x4t"),
        (0x03efca, 0x0403ca,  4, 4, true,  GAME_RED,  "game_medibot_4x4t"),
        (0x0403ca, 0x0425ca,  4, 4, true,  GAME_RED,  "game_ball_stuff_4x4t"),
        (0x0425ca, 0x04286a,  1, 1, false, GAME,      "game_arena_1x1"),
        (0x04286a, 0x0454ca,  4, 4, false, GAME_RED,  "game_arena_4x4"),

        // second set of sprites, near the end of the ROM.
        (0x0610c4, 0x0623c4, 2, 2, false, NON_GAME, "menu_font"),
        (0x0623c4, 0x067a44, 2, 2, false, MGMT,     "mgmt_background_2x2"),
        (0x067a44, 0x068244, 2, 2, false, MGMT,     "mgmt_lights_2x2"),
        (0x068244, 0x06da44, 4, 4, false, MGMT,     "mgmt_buttons_4x4"),
        (0x06da44, 0x072444, 4, 4, false, MGMT,     "mgmt_armour_4x4"),
        (0x072444, 0x072aa4, 1, 1, false, NON_GAME, "font_orange_1x1"),
        (0x072aa4, 0x072e44, 1, 1, false, NON_GAME, "font_title_top_1x1"),
        (0x072e44, 0x073284, 1, 1, false, NON_GAME, "font_title_bottom_1x1"),
	(0x073284, 0x0734e4, 1, 1, false, MGMT,     "font_mgr_xfer_gym_1x1"),
        (0x0734e4, 0x073964, 1, 1, false, MGMT,     "font_cash_1x1"),
	(0x073964, 0x073e04, 1, 1, false, MGMT,     "font_small_green_1x1"),
	(0x073e04, 0x074284, 1, 1, false, NON_GAME, "font_white_1x1"),
        (0x074284, 0x07e004, 2, 2, false, MGMT,     "mgmt_faces_6x6"),
    ] {
        let w = 36 / width;
        let img_data = &data[*start..*end];
        let palette_data = &data[*palette..];
        let img = build_image(img_data, w, palette_data, *width, *height, *transpose);
        img.save(Path::new(format!("{}/{}.png", OUT_DIR, name).as_str()))?;
    }

    Ok(())
}
