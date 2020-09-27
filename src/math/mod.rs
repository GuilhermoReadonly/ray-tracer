mod sphere;
mod vec3;

pub use sphere::*;
pub use vec3::*;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;

pub const TAU: f64 = 2.0 * PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * TAU / 360.0;
}
