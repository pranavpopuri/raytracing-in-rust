mod camera;
mod color;
mod common;
mod config;
mod hittable;
mod material;
mod ray;
mod stl;
mod vec3;

use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{
    config::{ASPECT_RATIO, Args, IMAGE_HEIGHT, IMAGE_WIDTH, SHOW_AXES},
    hittable::{HittableList, Sphere, add_axes},
};

use camera::Camera;
use color::Color;
use hittable::Hittable;
use image::{Rgb, RgbImage};
use material::{Dielectric, Lambertian};
use ray::Ray;
use vec3::Point3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(r, 0.001, common::INFINITY) {
        if let Some(scatter_rec) = hit_rec.mat.as_ref().unwrap().scatter(r, &hit_rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(0.8, 0.8, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn create_scene(world: &mut HittableList) {
    let water_mat = Arc::new(Dielectric::new(1.33, Color::new(0.6, 0.8, 1.0)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        water_mat,
    )));

    let island_material = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 0.5)));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -99.0, 0.0),
        100.0,
        island_material,
    )));

    let dragon = stl::models::dragon(Point3::new(0.0, 1.0, 0.0));
    world.add(dragon);

    let whale = stl::models::whale(Point3::new(10.0, 3.0, 10.0));
    world.add(whale);

    if SHOW_AXES {
        add_axes(world, 0.2, 2.0);
    }
}

fn create_camera() -> Camera {
    let lookfrom = Point3::new(15.0 * 3.0, 3.0, 3.0 * 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 30.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    cam
}

fn main() {
    let args = Args::parse();

    // World
    let mut world = HittableList::new();
    create_scene(&mut world);

    // Camera
    let cam = create_camera();

    // Render to image.png
    let start = Instant::now();
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let mut image = RgbImage::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for y in (0..IMAGE_HEIGHT).rev() {
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|x| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..args.samples {
                    let u = (x as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (y as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64; // Use y instead of j
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, args.maxdepth);
                }
                (x, pixel_color)
            })
            .collect();

        for (x, pixel_color) in pixel_colors {
            image.put_pixel(
                x as u32,
                (IMAGE_HEIGHT - y - 1) as u32,
                Rgb(color::color_to_array(pixel_color, args.samples)),
            );
        }

        bar.inc(1);
    }

    image.save(&args.out).unwrap();
    let end = Instant::now().duration_since(start);
    bar.finish();
    println!("Time taken: {}s", (end.as_micros() / 1000) as f64 / 1000.0);
}
