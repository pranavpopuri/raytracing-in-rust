use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horiz: Vec3,
    vert: Vec3,
}

/// TODO: make these constants better
impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horiz = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);

        // right hand rule means in front of camera is actually negative z
        let lower_left_corner =
            origin - horiz / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horiz,
            vert,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            // it's direction, so remove the fact that you start at origin:
            self.lower_left_corner + u * self.horiz + v * self.vert - self.origin,
        )
    }
}
