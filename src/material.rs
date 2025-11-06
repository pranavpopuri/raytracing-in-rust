use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3};

pub trait Material {
    /// attenuation is how much weaker the ray should be in strength (energy loss)
    /// scattered is in which way the reflected goes
    /// if it returns false, it means the ray was absorbed.
    /// TODO: make this return an Option<tuple>
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

/// TODO: organize into a folder
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    /// It can either scatter always, and attenuate by R
    /// or scatter with no attenuation, but absorb (1 - R)
    /// where R is probability it is absorbed
    /// **NOTE**: Albedo is attenuation!
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + vec3::rand_unit_vector();

        // if rand_unit = rec normal, you get a "zero vector"
        // which isn't good, so we just replace it with another one
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_dir);

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // TODO: why is v normalized? Isn't rec.normal already normalized? What?
        let reflected = vec3::reflect(vec3::norm(r_in.dir()), rec.normal);

        *attenuation = self.albedo;

        *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::rand_unit_vector());

        vec3::dot(scattered.dir(), rec.normal) > 0.0
    }
}
