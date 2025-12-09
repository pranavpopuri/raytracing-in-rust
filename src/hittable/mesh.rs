use crate::hittable::{HitRecord, Hittable, Triangle};
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default)]
pub struct Mesh {
    pub objects: Vec<Triangle>,
}

impl Mesh {
    pub fn new(objects: Vec<Triangle>) -> Self {
        Self { objects }
    }

    /// Map each point by a function
    pub fn map(&mut self, map: impl Fn(Point3) -> Point3) {
        for triangle in &mut self.objects {
            triangle.vertex0 = map(triangle.vertex0);
            triangle.vertex1 = map(triangle.vertex1);
            triangle.vertex2 = map(triangle.vertex2);
        }
    }

    pub fn center(&self) -> Point3 {
        let verts = &self.objects;
        // each object is a triangle
        // each triangle has three points
        // let's map objects so that it goes through each triangle, and finds the center of each triangle
        // then, find the center of all of those points

        let len = verts.len() as f64;
        let sum = verts
            .iter()
            .map(|triangle| (triangle.vertex0 + triangle.vertex1 + triangle.vertex2) / 3.0)
            .fold(Point3::default(), |prev, curr| prev + curr);

        sum / len
    }

    pub fn radius(&self) -> f64 {
        let verts = &self.objects;
        // each object is a triangle
        // each triangle has three points
        // let's map objects so that it goes through each triangle, and finds the center of each triangle
        // then, find the center of all of those points

        let len = verts.len() as f64;
        let sum: f64 = verts
            .iter()
            .map(|triangle| {
                (triangle.vertex0.length() + triangle.vertex1.length() + triangle.vertex2.length())
                    / 3.0
            })
            .sum();

        sum / len
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
