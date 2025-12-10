use clap::Parser;

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: i32 = 512;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 50;
pub const MAX_DEPTH: i32 = 50;
pub const OUTPUT_PATH: &'static str = "image.png";
pub const SHOW_AXES: bool = false;
pub const SHOW_DIAGONISTICS: bool = false;

// command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Output file
    #[arg(short, long, default_value_t = OUTPUT_PATH.to_string())]
    pub out: String,

    /// Number of bounces
    #[arg(short, long, default_value_t = MAX_DEPTH)]
    pub maxdepth: i32,

    /// Number of samples per pixel
    #[arg(short, long, default_value_t = SAMPLES_PER_PIXEL)]
    pub samples: i32,

    #[arg(short, long, default_value_t = SHOW_DIAGONISTICS)]
    pub verbose: bool,
}
