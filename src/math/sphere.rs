use super::Vec3;
use crate::{HitRecord, Hittable, Material, Ray};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn new_boxed(center: Vec3, radius: f64, material: M) -> Box<Self> {
        Box::new(Self::new(center, radius, material))
    }

    fn get_hit_record(&self, r: &Ray, t: f64) -> Option<HitRecord> {
        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        let hit_record: HitRecord = HitRecord::new(point, normal, t, front_face, &self.material);

        return Some(hit_record);
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return self.get_hit_record(r, temp);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return self.get_hit_record(r, temp);
            }
        }

        return None;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     //use rand::prelude::*;
//     use crate::Ray;
//     use test;

//     #[test]
//     fn hit_sphere_test() {
//         let origin = Vec3::new(0.0, 0.0, 0.0);
//         let direction = Vec3::new(0.0, 0.0, -1.0);
//         let ray = Ray::new(origin, direction);
//         let material = todo!();
//         let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material);

//         let result = sphere.hit(&ray, 0.0, 100.0);
//         let expected_result = Some(HitRecord::new(
//             Vec3::new(0.0, 0.0, -0.5),
//             Vec3::new(0.0, 0.0, 1.0),
//             0.5,
//             true,
//             &material
//         ));

//         // assert_eq!(result, expected_result);

//         let origin = Vec3::new(0.0, 0.0, 0.0);
//         let direction = Vec3::new(0.0, 0.0, -1.0);
//         let ray = Ray::new(origin, direction);
//         let material = todo!();
//         let sphere = Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.5, material);

//         let result = sphere.hit(&ray, 0.0, 100.0);
//         // let expected_result = None;

//         // assert_eq!(result, expected_result);
//     }
// }
