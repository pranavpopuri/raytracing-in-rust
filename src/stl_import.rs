use std::fs::OpenOptions;

use crate::vec3::Point3;

pub fn parse_stl(file: &str) -> Vec<stl_io::Triangle> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(format!("stl_folder/{file}"))
        .unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    let triangles = stl.into_triangle_vec();

    triangles
}

/// Take in a `stl_io::triangle``, and map it into an array of 3 points, with ops (such as rotation scaling)
pub fn map_stl_triangle(
    triangle: stl_io::Triangle,
    func: &dyn Fn(f64, f64, f64) -> (f64, f64, f64),
) -> (Point3, Point3, Point3) {
    let out = triangle.vertices.map(|v| {
        let (x, y, z) = func(v.0[0] as f64, v.0[1] as f64, v.0[2] as f64);
        Point3::new(x, y, z)
    });

    (out[0], out[1], out[2])
}
