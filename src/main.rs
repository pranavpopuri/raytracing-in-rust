use image::{Rgb, RgbImage};

fn main() {
    println!("Hello, world!");

    let mut img = RgbImage::new(32, 32);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }

    img.save("src/plus.png").unwrap();

    // let mut img = RgbImage::new(32, 32);

    // for x in 15..=17 {
    //     for y in 8..24 {
    //         img.put_pixel(x, y, Rgb([255, 0, 0]));
    //         img.put_pixel(y, x, Rgb([255, 0, 0]));
    //     }
    // }

    // img.save("src/plus.png").unwrap();
}
