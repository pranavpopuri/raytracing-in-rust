use std::sync::Arc;

use crate::vec3::{Vec3, Point3, dot, cross, unit_vector};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Triangle {
    pub vertex0: Point3,
    pub vertex1: Point3,
    pub vertex2: Point3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(vertex0: Point3, vertex1: Point3, vertex2: Point3, material: Arc<dyn Material>) -> Self {
        Self { vertex0, vertex1, vertex2, material }
    }

    fn normal(&self) -> Vec3 {
        unit_vector(cross(self.vertex1 - self.vertex0, self.vertex2 - self.vertex0))
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Möller–Trumbore intersection algorithm
        let eps = 1e-8;
        // Edges from vertex0
        let edge_v0_v1 = self.vertex1 - self.vertex0;
        let edge_v0_v2 = self.vertex2 - self.vertex0;

        // pvec is perpendicular to ray direction and edge_v0_v2
        let pvec = cross(ray.direction(), edge_v0_v2);
        let determinant = dot(edge_v0_v1, pvec);

        // If determinant is close to 0, the ray and triangle are parallel.
        if determinant.abs() < eps {
            return None;
        }

        let inv_determinant = 1.0 / determinant;
        let tvec = ray.origin() - self.vertex0;
        let barycentric_u = inv_determinant * dot(tvec, pvec);
        if barycentric_u < 0.0 || barycentric_u > 1.0 {
            return None;
        }

        let qvec = cross(tvec, edge_v0_v1);
        let barycentric_v = inv_determinant * dot(ray.direction(), qvec);
        if barycentric_v < 0.0 || barycentric_u + barycentric_v > 1.0 {
            return None;
        }

        // Compute intersection distance along the ray
        let ray_t = inv_determinant * dot(edge_v0_v2, qvec);
        if ray_t < t_min || ray_t > t_max {
            return None;
        }

        // Compute hit position and normal
        let hit_point = ray.at(ray_t);
        let triangle_normal = self.normal();

        let mut rec = HitRecord {
            p: hit_point,
            normal: Vec3::default(),
            mat: Some(self.material.clone()),
            t: ray_t,
            front_face: false,
        };

        rec.set_face_normal(ray, triangle_normal);
        Some(rec)
    }
}
