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
        let has_hit = self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);
        match has_hit {
            Some(t) => {
                let n: Vec3 = Vec3::unit(self.at(t) - Vec3::new(0.0, 0.0, -1.0));
                // we map each component of the normal vector from [-1, 1] to [0, 1] so it can fit in a Color vector
                Color::new_with_vec(0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0))
            }
            None => {
                let unit_direction: Vec3 = Vec3::unit(self.direction);
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::new_with_vec(
                    (1.0 - t) * Color::new(1.0, 1.0, 1.0).vec + t * Color::new(0.5, 0.7, 1.0).vec,
                )
            }
        }
    }

    fn hit_sphere(&self, center: Vec3, radius: f64) -> Option<f64> {
        let oc: Vec3 = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = Vec3::dot(&oc, &self.direction);
        let c = oc.length_squared() - radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use rand::prelude::*;
    use test;

    #[test]
    fn hit_sphere_test() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere_center = Vec3::new(0.0, 0.0, 0.0);
        let sphere_radius = 0.5;

        let result = ray.hit_sphere(sphere_center, sphere_radius);

        assert_eq!(result, Some(-0.5))
    }
}
