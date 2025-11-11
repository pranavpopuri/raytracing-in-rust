pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: i32 = 512;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 100;
pub const MAX_DEPTH: i32 = 50;
pub const IMAGE_PATH: &'static str = "image.png";
