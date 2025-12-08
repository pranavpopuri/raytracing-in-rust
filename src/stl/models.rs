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

    let scale = 1.0 / 250.0;
    let mesh = Box::new(import_stl("grass.stl", material, &|x, y, z| {
        (
            scale * x + pos.x(),
            scale * z + pos.y(),
            scale * -y + pos.z(),
        )
    }));

    mesh
}

pub fn rock(pos: Point3) -> Box<Mesh> {
    let material = Arc::new(Lambertian::new(Color::new(0.3, 0.3, 0.3)));

    let scale = 1.0 / 153.0;
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
