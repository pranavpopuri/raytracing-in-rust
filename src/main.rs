use crate::{
    camera::Camera,
    color::{Color, write_color},
    common::rand_double,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

use std::time::Instant;

mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod sphere;
// mod save_ppm;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        // map from -1 to 1 to 0 to 1
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = vec3::norm(r.dir());

    // maps y which should go from [-1, 1] to [0, 1]
    let t = 0.5 * (unit_dir.y() + 1.0);

    // defines a gradient from white to blue (bottom to top)
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

/// TODO: make it work with image crate
fn main() {
    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    // camera
    let cam = Camera::new();

    // top of y == 0 in file, but top of y == height in math space
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rProgress: {}/{}", IMAGE_HEIGHT - 1 - y, IMAGE_HEIGHT - 1);
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                // u goes from 0 to 1
                // because horiz is width IMAGE_WIDTH
                // TODO: if division is so slow, why don't we change this?
                // instead, just let x vary, and let horiz be (1, 0, 0)
                let u = (x as f64 + rand_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rand_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(&mut std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
            // let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            // let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;

            // let r = Ray::new(
            //     origin,
            //     lower_left_corner + u * horizontal + v * vertical - origin,
            // );

            // let pixel_color = ray_color(&r, &world);
            // write_color(&mut std::io::stdout(), pixel_color);
        }
    }
}
