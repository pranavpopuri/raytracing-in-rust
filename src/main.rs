mod camera;
mod color;
mod common;
mod config;
mod hittable;
mod material;
mod ray;
mod stl_import;
mod vec3;

use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{
    config::{ASPECT_RATIO, Args, IMAGE_HEIGHT, IMAGE_WIDTH, SHOW_AXES, SHOW_DIAGONISTICS},
    hittable::{HittableList, Mesh, Sphere, Triangle, add_axes},
    material::Material,
    stl_import::{map_stl_triangle, parse_stl},
};

use camera::Camera;
use color::Color;
use hittable::Hittable;
use image::{Rgb, RgbImage};
use material::{Dielectric, Lambertian, Metal};
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
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let material4 = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let tri = Triangle::new(
        Point3::new(-1.0, 0.0, -3.0),
        Point3::new(1.0, 0.0, -3.0),
        Point3::new(0.0, 1.0, -3.0),
        material4,
    );

    world.add(Box::new(tri));

    world
}

fn stl_mesh(
    file: &str,
    mat: Arc<dyn Material>,
    map: &dyn Fn(f64, f64, f64) -> (f64, f64, f64),
) -> Mesh {
    let triangles: Vec<_> = parse_stl(file)
        .into_iter()
        .map(|stl_triangle| {
            let (p1, p2, p3) = map_stl_triangle(
                stl_triangle,
                // rotate around the x axis
                // and scale by the scale amount
                map,
            );

            Triangle::new(p1, p2, p3, mat.clone())
        })
        .collect();

    Mesh::new(triangles)
}

fn get_center(mesh: &Box<Mesh>) -> Point3 {
    let verts = &mesh.objects;
    // each object is a triangle
    // each triangle has three points
    // let's map objects so that it goes through each triangle, and finds the center of each triangle
    // then, find the center of all of those points

    let len = verts.len() as f64;
    let sum = verts
        .iter()
        .map(|triangle| (triangle.vertex0 + triangle.vertex1 + triangle.vertex2) / 3.0)
        .fold(Point3::default(), |prev, curr| prev + curr);

    sum / len
}

fn avg_mag(mesh: &Box<Mesh>) -> f64 {
    let verts = &mesh.objects;
    // each object is a triangle
    // each triangle has three points
    // let's map objects so that it goes through each triangle, and finds the center of each triangle
    // then, find the center of all of those points

    let len = verts.len() as f64;
    let sum: f64 = verts
        .iter()
        .map(|triangle| {
            (triangle.vertex0.length() + triangle.vertex1.length() + triangle.vertex2.length())
                / 3.0
        })
        .sum();

    sum / len
}

fn main() {
    let args = Args::parse();

    // World
    let mut world = random_scene();

    let mat = Arc::new(Metal::new(Color::new(0.2, 0.8, 0.1), 0.2));

    let scale = 1.0 / 40.0;
    let mesh = Box::new(stl_mesh("small_dragon.stl", mat, &|x, y, z| {
        (scale * x, scale * z, scale * -y)
    }));

    let center = get_center(&mesh);

    if SHOW_DIAGONISTICS {
        println!("Radius: {}", avg_mag(&mesh));
        println!("Center: {center}");
    }

    world.add(mesh);

    if SHOW_AXES {
        add_axes(&mut world, 0.2, 2.0);
    }

    // Camera
    let lookfrom = Point3::new(15.0, 6.0, 3.0);
    let lookat = center;
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
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

    // Render to image.ppm
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
