use crate::{
    color::{Color, write_color},
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod color;
// mod save_ppm;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn ray_color(r: &Ray) -> Color {
    let unit_dir = vec3::unit_vector(r.dir());

    // maps y which should go from [-1, 1] to [0, 1]
    let t = 0.5 * (unit_dir.y() + 1.0);

    // defines a gradient from white to blue (bottom to top)
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

/// TODO: make it work with image crate
fn main() {
    // let mut img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    // right hand rule means in front of camera is actually negative z
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    // top of y == 0 in file, but top of y == height in math space
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rProgress: {}/{}", IMAGE_HEIGHT - 1 - y, IMAGE_HEIGHT - 1);
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r);
            write_color(&mut std::io::stdout(), pixel_color);
        }
    }
}
