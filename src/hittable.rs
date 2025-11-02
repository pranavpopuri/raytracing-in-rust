use crate::{
    ray::Ray,
    vec3::{self, Point3, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // if its negative, then that means they are going opposite ways.
        // if it's going opposite ways, then that means the ray is outside (bounced away)
        // if it's going the same way, that means the ray is inside, (going same dir as normal which is away from center)
        self.front_face = vec3::dot(r.dir(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            // make it point against the ray
            // since we know front face is false, we are inside the sphere.
            -outward_normal
        };
    }
}

pub trait Hittable {
    /// t is valid iff t_min <= t <= t_max (i.e. t is positive)
    /// it's kind of a "if the object is too far, don't render it"
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
