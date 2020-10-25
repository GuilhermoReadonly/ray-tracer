use crate::{math::Vec3, Color, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal + Vec3::new_random_unit();
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        // let scatter_direction = hit_record.normal + Vec3::new_random_unit();
        // let scattered = Ray::new(hit_record.point, scatter_direction);
        // let attenuation = self.albedo;
        // Some((scattered, attenuation))

        let reflected = Vec3::reflect(&Vec3::unit(ray_in.direction), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);

        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
