use crate::color::{Color, write_color};

mod color;
// mod save_ppm;
mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

/// TODO: make it work with image crate
fn main() {
    println!("Hello, world!");

    // let mut img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rProgress: {y}/{}", IMAGE_HEIGHT - 1);
        for x in 0..IMAGE_WIDTH {
            let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let color = Color::new(r, g, b);
            write_color(&mut std::io::stdout(), color);
        }
    }
}
