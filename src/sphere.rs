use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{self, Point3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    /// Equation of sphere: `(P - C) * (P - C) = r ^ 2`
    /// Ray: `P(t) = at + b`
    ///
    /// Polynomial:
    /// `a = b * b`
    /// `b = 2b * (a - C)`
    /// `c = (a - C) * (a - C) - r ^ 2`
    /// `oc = (a - C)`
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig() - self.center;
        let a = ray.dir().length_squared();
        // optimization: use b/2, and it simplifies operations
        let half_b = vec3::dot(oc, ray.dir());
        let c = oc.length_squared() - self.radius * self.radius;

        // we only care that there *is* a solution:
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;

        // try the other root if its invalid
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        true
    }
}
