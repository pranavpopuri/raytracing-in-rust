use std::io::Write;

use crate::vec3::Vec3;

// Type alias
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // map to [0, 255]
    // don't use 256 because of rounding error, and don't use 255 because it rounds down
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
