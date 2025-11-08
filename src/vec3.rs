use core::f64;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::common::{rand_double, rand_range};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn rand() -> Vec3 {
        Vec3::new(rand_double(), rand_double(), rand_double())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        // use the std eps, which is 1.0e-7 > 1.0e-8
        let eps = f64::EPSILON;
        self.e[0].abs() < eps && self.e[1].abs() < eps && self.e[2].abs() < eps
    }
}

// Type alias
pub type Point3 = Vec3;

// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

/// TODO: make this an instance method
pub fn norm(v: Vec3) -> Vec3 {
    v / v.length()
}

/// Random vec within unit sphere
fn rand_in_unit_sphere() -> Vec3 {
    // TODO: make this a better algorithm (benchmark)
    loop {
        let p = Vec3::rand_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

/// Random point on a unit sphere, if you offset that with normal
/// then you get lambertian scattering
/// Lambertian scattering
/// https://en.wikipedia.org/wiki/Lambertian_reflectance
pub fn rand_unit_vector() -> Vec3 {
    norm(rand_in_unit_sphere())
}

/// Remember, `projn v = (v * n / n * n)n`
/// But since `n` is a unit, `(v * n)n` is sufficient as a projection onto n
/// which is `b` which subtracts v to create the reflection
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

/// Snell's law: (where t' is the refracted ray)
/// eta * sin(t) = eta' * sin(t')
/// sin(t') = eta / eta' * sin(t)
///
/// Now, the refracted ray R' = Rperp' + R||', where n is a normal
/// Rperp' = eta / eta' * (R + n * cos(t))
/// R||' = -n * sqrt(1 - |Rperp'|^2)
///
/// Now, since n is the opposite direction of R, but both are unit:
/// a dot b = |a||b| cos(t), a dot b = cos(t)
/// So, Rperp' = eta / eta' * (R + n * (-n * R))
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt((1.0 - r_out_perp.length_squared()).abs()) * n;

    r_out_perp + r_out_parallel
}
