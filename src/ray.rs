use crate::{math::Sphere, math::Vec3, Color};

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
        let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let has_hit = sphere.hit(&self, 0.0, 100.0);
        match has_hit {
            Some(hit_record) => {
                let n: Vec3 = hit_record.normal;
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;
    //use rand::prelude::*;
    use test;

    #[test]
    fn ray_color_test() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let result = ray.ray_color();

        assert_eq!(
            result,
            Color {
                vec: Vec3::new(0.5, 0.5, 1.0)
            }
        );
    }
}
