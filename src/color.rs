use std::io::Write;

use crate::vec3::Vec3;

// Type alias
pub type Color = Vec3;

/// TODO: move to a config
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // multiplication is faster than division
    // this averages out
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // map to [0, 255]
    // don't use 256 because of rounding error, and don't use 255 because it rounds down
    writeln!(
        out,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    )
    .expect("writing color");
}
