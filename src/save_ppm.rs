use image::{ImageBuffer, Rgb};
use std::fs;

pub fn save(file: &str, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(), std::io::Error> {
    let width = img.width();
    let height = img.height();

    let mut out = String::new();

    out += &format!("P3\n{width} {height}\n255\n");

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = img.get_pixel(y, x).0;
            out += &format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]);
        }
    }

    fs::write(file, out)?;

    Ok(())
}
