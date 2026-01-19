pub mod color;
pub mod ppm;

pub type Vec3 = nalgebra::Vector3<f32>;

pub const  ZERO: Vec3 = Vec3::new(0f32, 0f32, 0f32);

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random_range(0.0..1.0)
}