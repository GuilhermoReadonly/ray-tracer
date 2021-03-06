use crate::{
    math::{self, Vec3},
    Color, Material, World,
};
// use std::fmt::Debug;

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

    pub fn ray_color<F>(&self, world: &World<F>, depth: u32) -> Color
    where
        F: Fn(&Ray) -> Color,
    {
        match (world.hit(&self, 0.001, math::INFINITY), depth) {
            // If the ray bounced enougth (depth = 0) we consider it is now completly black and we stop here
            (_, 0) => Color::new(0.0, 0.0, 0.0),

            // If the ray hit something ,we scater it and decrement the depth counter
            (Some(hit_record), depth) => {
                if let Some((scattered, attenuation)) =
                    hit_record.material.scatter(&self, &hit_record)
                {
                    let emitted = hit_record.material.emitted();
                    emitted + attenuation * scattered.ray_color(world, depth - 1)
                } else {
                    hit_record.material.emitted()
                }
            }

            // If the ray hit nothing we draw the background
            (None, _) => (&world.background)(&self),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// #[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     //use rand::prelude::*;
//     use crate::math::Sphere;
//     use test;

//     #[test]
//     fn hittable_list_test() {
//         let origin = Vec3::new(0.0, 0.0, 0.0);
//         let direction = Vec3::new(0.0, 0.0, -1.0);
//         let ray = Ray::new(origin, direction);

//         let mut hittable_list = HittableList::new();

//         let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5);
//         let sphere1 = Box::new(sphere1);
//         hittable_list.add(sphere1);

//         let result = hittable_list.hit(&ray, 0.0, 100.0);
//         let expected_result = Some(HitRecord::new(
//             Vec3::new(0.0, 0.0, -1.5),
//             Vec3::new(0.0, 0.0, 1.0),
//             1.5,
//             true,
//         ));

//         assert_eq!(result, expected_result);

//         let sphere2 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
//         let sphere2 = Box::new(sphere2);
//         hittable_list.add(sphere2);

//         let result = hittable_list.hit(&ray, 0.0, 100.0);
//         let expected_result = Some(HitRecord::new(
//             Vec3::new(0.0, 0.0, -0.5),
//             Vec3::new(0.0, 0.0, 1.0),
//             0.5,
//             true,
//         ));

//         // assert_eq!(result, expected_result);

//         hittable_list.clear();
//         assert_eq!(hittable_list.objects.len(), 0);
//     }

//     #[test]
//     fn ray_color_test() {
//         let origin = Vec3::new(0.0, 0.0, 0.0);
//         let direction = Vec3::new(0.0, 0.0, -1.0);
//         let ray = Ray::new(origin, direction);

//         let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
//         let mut world = HittableList::new();
//         world.add(Box::new(sphere));
//         let result = ray.ray_color(&world, 100, 10);

//         assert_eq!(result.vec.z, 0.5);
//     }
// }
