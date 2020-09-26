mod sphere;
mod vec3;

pub use sphere::*;
pub use vec3::*;

pub const PI: f32 = std::f32::consts::PI;
pub const INFINITY: f32 = std::f32::INFINITY;
pub const TAU: f32 = 2.0 * PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * TAU / 360.0;
}
