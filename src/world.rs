use crate::{Color, HitRecord, Hittable, Ray};
// use std::fmt::Debug;

pub struct World<F>
where
    F: Fn(&Ray) -> Color,
{
    pub objects: Vec<Box<dyn Hittable>>,
    pub background: F,
}

impl<F> World<F>
where
    F: Fn(&Ray) -> Color,
{
    pub fn new(background: F) -> Self {
        World {
            objects: vec![],
            background,
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<F> Hittable for World<F>
where
    F: Fn(&Ray) -> Color,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.objects.iter() {
            if let Some(hit) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
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
