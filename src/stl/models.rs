//! Store how to import STL files, and any transformations needed on them

use std::sync::Arc;

use super::import_stl;
use crate::{
    color::Color,
    hittable::Mesh,
    material::{Lambertian, Metal},
    vec3::Point3,
};

pub fn tree(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.1)));

    let scale = 1.0 / 10.0;
    let mesh = Box::new(import_stl("lowpoly_tree.stl", material, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * y + pos.y(),
            scale * z + pos.z(),
        )
    }));

    mesh
}

pub fn grass(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.1)));

    let scale = 1.0 / 1000.0;
    let mesh = Box::new(import_stl("grass.stl", material, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * z + pos.y() - -0.5,
            scale * -y + pos.z(),
        )
    }));

    mesh
}

pub fn rock(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(0.3, 0.3, 0.3)));

    let scale = 1.0 / 400.0;
    let mesh = Box::new(import_stl("rock.stl", material, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * y + pos.y(),
            scale * z + pos.z(),
        )
    }));

    mesh
}

pub fn dragon(pos: Point3) -> Box<Mesh> {
    let mat = Arc::new(Metal::new(Color::new(0.2, 0.8, 0.1), 0.1));

    let scale = 1.0 / 40.0;
    let mesh = Box::new(import_stl("small_dragon.stl", mat, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * z + pos.y(),
            scale * -y + pos.z(),
        )
    }));

    mesh
}

pub fn cs128(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.1)));

    let scale = 1.0 / 10.0;
    let mesh = Box::new(import_stl("simplify_cs128.stl", material, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * y + pos.y(),
            scale * z + pos.z(),
        )
    }));

    mesh
}

fn rotate_by(x: f64, y: f64, z: f64, deg: f64) -> (f64, f64, f64) {
    // Angle in radians
    let theta = deg.to_radians();

    // Precompute cos and sin
    let cos_theta = theta.cos(); // ≈ -0.5
    let sin_theta = theta.sin(); // ≈ 0.866

    let new_x = x;
    let new_y = y * cos_theta - z * sin_theta;
    let new_z = y * sin_theta + z * cos_theta;

    (new_x, new_y, new_z)
}

pub fn whale(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(0.22, 0.42, 0.75)));

    let scale = 0.5;
    let mesh = Box::new(import_stl("whale.stl", material, &|x, y, z| {
        let (x, y, z) = rotate_by(x, y, z, 80f64);
        (
            scale * x + pos.x(),
            scale * y + pos.y(),
            scale * z + pos.z(),
        )
    }));

    mesh
}
