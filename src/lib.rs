use image::Color;
use math::Vec3;

pub mod error;
pub mod image;
pub mod math;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn at(self: &Self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    fn ray_color(&self) -> Color {
        let unit_direction: Vec3 = Vec3::unit(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new_with_vec(
            (1.0 - t) * Color::new(1.0, 1.0, 1.0).vec + t * Color::new(0.5, 0.7, 1.0).vec,
        )
    }
}
