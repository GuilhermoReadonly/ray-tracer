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
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&Vec3::unit(ray_in.direction), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::new_random_in_unit_sphere(),
        );

        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dielectric {
    pub albedo: Color,
    pub ir: f64,
}

impl Dielectric {
    pub fn new(albedo: Color, ir: f64) -> Dielectric {
        Dielectric { albedo, ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {

        let refraction_ratio = if hit_record.front_face  {1.0/self.ir} else {self.ir};

        let unit_direction = Vec3::unit(ray_in.direction);

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else{
            Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);

        Some((scattered, self.albedo))
    }
}
