//
// Cell extractor
//
// Breaks down a binary file into Megadrive-style 8x8 image 'cells',
// and constructs an image from that. Use to find bitmaps in a
// Megadrive ROM.
//

use image::GrayImage;
use std::path::Path;

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
// Entry point
//

fn main() {
    println!("Hello, world!");

    let mut my_image = Image::new(64, 64);
    for i in 16..48 {
        my_image.set_pixel(i, i + 10, 128 + i as u8);
    }

    my_image.save(Path::new("my_image.png"));
}
