use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
        let mut has_hit = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            // closest_so_far will decrease, which means you don't render objects behind it
            if obj.hit(ray, t_min, closest_so_far, &mut temp) {
                has_hit = true;
                closest_so_far = temp.t;
                *rec = temp.clone();
            }
        }

        has_hit
    }
}
