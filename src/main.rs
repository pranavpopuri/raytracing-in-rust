mod camera;
mod color;
mod common;
mod config;
mod hittable;
mod material;
mod ray;
mod vec3;

use std::sync::Arc;
use std::time::Instant;

use crate::{
    config::*,
    hittable::{HittableList, Mesh, Sphere, Triangle, add_axes},
    material::Material,
};

use camera::Camera;
use color::Color;
use hittable::Hittable;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use rayon::prelude::*;
use vec3::Point3;
use clap::{Parser, Subcommand};

use std::fs::OpenOptions;

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

fn stl_mesh(file: &str, mat: Arc<dyn Material>, scale: f64) -> Mesh {
    let mut file = OpenOptions::new()
        .read(true)
        .open(format!("stl_folder/{file}"))
        .unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    let triangles = stl.into_triangle_vec();
    println!("Triangle count: {}", triangles.len());

    let mut mesh = Mesh::new();

    for triangle_parsed in triangles {
        let vertices = triangle_parsed.vertices;
        let triangle = Triangle::new(
            Into::<Point3>::into(vertices[0]) * scale,
            Into::<Point3>::into(vertices[1]) * scale,
            Into::<Point3>::into(vertices[2]) * scale,
            mat.clone(),
        );
        mesh.add(triangle);
    }

    mesh
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

// command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Default {
      output_path: String
    },
    Parse {
      input_path: String,
      output_path: String
    },
    Full {
      input_path: String,
      output_path: String,
      max_width: i32,
      samples_per_pixel: i32,
      max_depth: i32
    }
}

fn main() {
    let mut input = String::from("small_dragon.stl");
    let mut width = IMAGE_WIDTH;
    let mut height = IMAGE_HEIGHT;
    let mut output = String::from(IMAGE_PATH);
    let mut depth = MAX_DEPTH;
    let mut samples = SAMPLES_PER_PIXEL;

    let args = Args::parse();

    match args.command {
      Some(Commands::Default { output_path }) => {
        output = output_path;
      }

      Some(Commands::Parse { input_path, output_path }) => {
        input = input_path;
        output = output_path;
      }

      Some(Commands::Full { input_path, output_path, max_width, samples_per_pixel, max_depth }) => {
        input = input_path;
        output = output_path;
        width = max_width;
        height = (width as f64 / ASPECT_RATIO) as i32;
        samples = samples_per_pixel;
        depth = max_depth;
      }

      None => {}
    }

    // World
    let mut world = HittableList::new();

    let mat = Arc::new(Metal::new(Color::new(0.2, 0.8, 0.1), 0.2));
    let mesh = Box::new(stl_mesh(&input, mat, 1.0 / 40.0));

    let center = get_center(&mesh);
    // println!("{}", avg_mag(&mesh));
    // world.add(mesh);

    println!("{center}");

    add_axes(&mut world, 0.2, 1.0);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
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
    let bar = ProgressBar::new(height as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let mut image = RgbImage::new(width as u32, height as u32);

    for y in (0..height).rev() {
        let pixel_colors: Vec<_> = (0..width)
            .into_par_iter()
            .map(|x| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples {
                    let u = (x as f64 + common::random_double()) / (width - 1) as f64;
                    let v = (y as f64 + common::random_double()) / (height - 1) as f64; // Use y instead of j
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, depth);
                }
                (x, pixel_color)
            })
            .collect();

        for (x, pixel_color) in pixel_colors {
            image.put_pixel(
                x as u32,
                (height - y - 1) as u32,
                Rgb(color::color_to_array(pixel_color, samples)),
            );
        }

        bar.inc(1);
    }

    image.save(&output).unwrap();
    let end = Instant::now().duration_since(start);
    bar.finish();
    println!("Time taken: {}s", (end.as_micros() / 1000) as f64 / 1000.0);
}
