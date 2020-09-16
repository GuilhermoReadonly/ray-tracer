use super::Vec3;
use crate::{HitRecord, Hittable, Ray};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    fn get_hit_record(&self, r: &Ray, t: f64) -> Option<HitRecord>{
        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else { -outward_normal};

        let hit_record: HitRecord = HitRecord {
            point,
            normal,
            t,
            front_face,
        };

        return Some(hit_record);
    }
}

impl Hittable for Sphere {
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

#[cfg(test)]
mod tests {
    use super::*;
    //use rand::prelude::*;
    use crate::Ray;
    use test;

    #[test]
    fn hit_sphere_test() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.0, 100.0);
        let expected_result = Some(HitRecord {
            point: Vec3::new(0.0, 0.0, -0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            t: 0.5,
            front_face: true,
        });

        assert_eq!(result, expected_result);

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.5);

        let result = sphere.hit(&ray, 0.0, 100.0);
        let expected_result = None;

        assert_eq!(result, expected_result);
    }
}
