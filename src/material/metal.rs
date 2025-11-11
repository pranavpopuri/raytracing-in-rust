use super::Material;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::ScatterRecord;
use crate::ray::Ray;
use crate::vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());

        if vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
