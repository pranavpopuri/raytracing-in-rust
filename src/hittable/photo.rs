use std::sync::Arc;

use image::{ImageBuffer, ImageReader, Rgba};

use crate::hittable::{HitRecord, Hittable, Triangle};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Photo {
    top_left: Point3,
    width: f64,
    height: f64,
    image_width: u32,
    image_height: u32,

    /// 'x'
    u: Point3,
    /// 'y'
    v: Point3,
    rect: (Triangle, Triangle),
    pixels: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Photo {
    pub fn new(
        file: &str,
        top_left: Point3,
        width: f64,
        height: f64,
        u: Point3,
        v: Point3,
        mat: Arc<dyn Material>,
    ) -> Self {
        let image = ImageReader::open(file).unwrap().decode().unwrap();
        let pixels = image.to_rgba8();

        let p0 = top_left;
        let p1 = top_left + width * u; // top right
        let p2 = top_left - height * v; // bottom left
        let p3 = top_left + width * u - height * v; // bottom right

        Self {
            top_left,
            width,
            height,
            u,
            v: -v, // make it point downwards
            image_width: pixels.width(),
            image_height: pixels.height(),
            rect: (
                Triangle::new(p0, p1, p2, mat.clone()),
                Triangle::new(p2, p1, p3, mat.clone()),
            ),
            pixels,
        }
    }
}

impl Hittable for Photo {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let record = self
            .rect
            .0
            .hit(ray, t_min, t_max)
            .or_else(|| self.rect.1.hit(ray, t_min, t_max));

        if let Some(record) = record {
            let rel = record.p - self.top_left;

            let u = vec3::dot(rel, self.u) / self.width;
            let v = vec3::dot(rel, self.v) / self.height;

            let x = (u * (self.image_width as f64)) as u32;
            let y = (v * (self.image_height as f64)) as u32;

            let pixel = self.pixels.get_pixel(x, y);
            if pixel.0[3] == 0 {
                return None;
            }

            return Some(record);
        }

        None
    }
}
