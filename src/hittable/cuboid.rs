use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HittableList, Mesh, Triangle},
    material::{Lambertian, Material},
    vec3::Point3,
};

pub fn new_cuboid(
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    h: f64,
    d: f64,
    mat: Arc<dyn Material>,
) -> Box<Mesh> {
    let v0 = Point3::new(x, y, z);
    let v1 = Point3::new(x + w, y, z);
    let v2 = Point3::new(x + w, y + h, z);
    let v3 = Point3::new(x, y + h, z);

    let v4 = Point3::new(x, y, z + d);
    let v5 = Point3::new(x + w, y, z + d);
    let v6 = Point3::new(x + w, y + h, z + d);
    let v7 = Point3::new(x, y + h, z + d);

    let objects = vec![
        // Front face
        Triangle::new(v0, v1, v2, mat.clone()),
        Triangle::new(v0, v2, v3, mat.clone()),
        // Back face
        Triangle::new(v5, v4, v7, mat.clone()),
        Triangle::new(v5, v7, v6, mat.clone()),
        // Left face
        Triangle::new(v4, v0, v3, mat.clone()),
        Triangle::new(v4, v3, v7, mat.clone()),
        // Right face
        Triangle::new(v1, v5, v6, mat.clone()),
        Triangle::new(v1, v6, v2, mat.clone()),
        // Top face
        Triangle::new(v3, v2, v6, mat.clone()),
        Triangle::new(v3, v6, v7, mat.clone()),
        // Bottom face
        Triangle::new(v0, v4, v5, mat.clone()),
        Triangle::new(v0, v5, v1, mat.clone()),
    ];

    Box::new(Mesh::new(objects))
}

/// x red, y green, z blue
pub fn add_axes(hittable_list: &mut HittableList, thickness: f64, length: f64) {
    let xmat = Arc::new(Lambertian::new(Color::new(0.8, 0.0, 0.0)));
    let ymat = Arc::new(Lambertian::new(Color::new(0.0, 0.8, 0.0)));
    let zmat = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.8)));

    // === X AXIS ===
    // Shaft
    hittable_list.add(new_cuboid(
        0.0,
        -thickness / 2.0,
        -thickness / 2.0,
        0.9 * length,
        thickness,
        thickness,
        xmat.clone(),
    ));
    // Head
    hittable_list.add(new_cuboid(
        0.9 * length,
        -thickness,
        -thickness,
        0.1 * length,
        2.0 * thickness,
        2.0 * thickness,
        xmat,
    ));

    // === Y AXIS ===
    // Shaft
    hittable_list.add(new_cuboid(
        -thickness / 2.0,
        0.0,
        -thickness / 2.0,
        thickness,
        0.9 * length,
        thickness,
        ymat.clone(),
    ));
    // Head
    hittable_list.add(new_cuboid(
        -thickness,
        0.9 * length,
        -thickness,
        2.0 * thickness,
        0.1 * length,
        2.0 * thickness,
        ymat,
    ));

    // === Z AXIS ===
    // Shaft
    hittable_list.add(new_cuboid(
        -thickness / 2.0,
        -thickness / 2.0,
        0.0,
        thickness,
        thickness,
        0.9 * length,
        zmat.clone(),
    ));
    // Head
    hittable_list.add(new_cuboid(
        -thickness,
        -thickness,
        0.9 * length,
        2.0 * thickness,
        2.0 * thickness,
        0.1 * length,
        zmat,
    ));
}
