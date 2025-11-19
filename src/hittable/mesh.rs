use crate::hittable::{HitRecord, Hittable, Triangle};
use crate::ray::Ray;

#[derive(Default)]
pub struct Mesh {
    pub objects: Vec<Triangle>,
}

impl Mesh {
    pub fn new(objects: Vec<Triangle>) -> Self {
        Self { objects }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
