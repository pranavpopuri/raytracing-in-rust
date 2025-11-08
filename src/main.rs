use std::rc::Rc;

use crate::{
    camera::Camera,
    color::{Color, write_color},
    common::rand_double,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::Point3,
};

mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod sphere;
// mod save_ppm;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();

    // Rays w/ t close to zero, might
    // accidentally hit the surface it just reflected. BAD
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();

        if rec
            .mat // Option<Rc<dyn Material>>
            .as_ref() // Option<&Rc<dyn Material>>
            .unwrap() // &Rc<dyn Material>
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        // this ray was absorbed
        return Color::new(0.0, 0.0, 0.0);
    }

    // here, the ray didn't hit anything, i.e. it hit the sky
    let unit_dir = vec3::norm(r.dir());

    // maps y which should go from [-1, 1] to [0, 1]
    let t = 0.5 * (unit_dir.y() + 1.0);

    // defines a gradient from white to blue (bottom to top)
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

/// TODO: make it work with image crate
fn main() {
    // world
    // TODO: move to its own function
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Dielectric::new(1.5));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    // let material_top = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        -0.4,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, 1.0, -1.0),
    //     0.5,
    //     material_top,
    // )));

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

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(&mut std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
