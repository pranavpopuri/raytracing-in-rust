use crate::{color::Color, common, hittable::HitRecord, ray::Ray, vec3};

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

pub struct Dielectric {
    ir: f64, // Index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // just magic lmao
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::norm(r_in.dir());

        let cos_t = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_t = f64::sqrt(1.0 - cos_t * cos_t);

        // Total internal reflection happens when
        // eta / eta' * sin(t) > 1
        // when sin(t') has no solution
        let cannot_refract = refraction_ratio * sin_t > 1.0;
        let dir = if cannot_refract
            || Self::reflectance(cos_t, refraction_ratio) > common::rand_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, dir);
        true
    }
}
