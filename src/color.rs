use crate::common;
use crate::vec3::Vec3;

// Type alias
pub type Color = Vec3;

pub fn color_to_array(color: Color, samples: i32) -> [u8; 3] {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    [
        (256.0 * common::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * common::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * common::clamp(b, 0.0, 0.999)) as u8,
    ]
}
