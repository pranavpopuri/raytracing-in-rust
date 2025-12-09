use rand::Rng;

// Constants

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Return a random real in [0.0, 1.0)
pub fn random_double() -> f64 {
    rand::rng().random()
}

/// Return a random real in [min, max)
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
