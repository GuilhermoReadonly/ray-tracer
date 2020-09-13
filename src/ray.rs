use crate::{Color, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self: &Self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self) -> Color {
        if self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5) {
            Color::new(1.0, 0.0, 0.0)
        } else {
            let unit_direction: Vec3 = Vec3::unit(&self.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::new_with_vec(
                (1.0 - t) * Color::new(1.0, 1.0, 1.0).vec + t * Color::new(0.5, 0.7, 1.0).vec,
            )
        }
    }

    fn hit_sphere(&self, center: Vec3, radius: f64) -> bool {
        let oc: Vec3 = self.origin - center;
        let a = Vec3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vec3::dot(&oc, &self.direction);
        let c = Vec3::dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
