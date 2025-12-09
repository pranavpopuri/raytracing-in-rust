use std::fs::OpenOptions;
use std::sync::Arc;

use crate::{
    config::SHOW_DIAGONISTICS,
    hittable::{Mesh, Triangle},
    material::Material,
    vec3::Point3,
};

/// Import an stl file and turn it into a `Mesh`
/// There is an map function to perform transformations
pub fn import_stl(
    file: &str,
    mat: Arc<dyn Material>,
    map: &dyn Fn(f64, f64, f64) -> (f64, f64, f64),
    should_center: bool,
) -> Mesh {
    let triangles: Vec<_> = parse_stl(file)
        .into_iter()
        .map(|stl_triangle| {
            let (p1, p2, p3) = map_stl_triangle(stl_triangle);

            Triangle::new(p1, p2, p3, mat.clone())
        })
        .collect();

    if SHOW_DIAGONISTICS {
        println!("{file} Triangles: {}", triangles.len());
    }

    let mut mesh = Mesh::new(triangles);
    let center = mesh.center();

    if should_center {
        mesh.map(|point| point - center);
    }

    mesh.map(|point| {
        let (x, y, z) = map(point.x(), point.y(), point.z());
        Point3::new(x, y, z)
    });

    mesh
}

/// Get the `Vec<stl_io::Triangle>` from an stl file
fn parse_stl(file: &str) -> Vec<stl_io::Triangle> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(format!("stl_folder/{file}"))
        .unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    let triangles = stl.into_triangle_vec();

    triangles
}

/// Take in a `stl_io::triangle`, and map it into a tuple of 3 points
fn map_stl_triangle(triangle: stl_io::Triangle) -> (Point3, Point3, Point3) {
    let out = triangle.vertices.map(|v| {
        let (x, y, z) = (v.0[0] as f64, v.0[1] as f64, v.0[2] as f64);
        Point3::new(x, y, z)
    });

    (out[0], out[1], out[2])
}
