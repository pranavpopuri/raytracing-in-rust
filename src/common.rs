use rand::Rng;
use std::f64::consts::PI;

pub fn deg_to_rad(degs: f64) -> f64 {
    degs * PI / 180.0
}

/// Returns random [0, 1)
pub fn rand_double() -> f64 {
    rand::rng().random()
}

/// Returns random [min, max)
pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_double()
}
