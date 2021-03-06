use crate::{math::Vec3, Color, HitRecord, Ray};
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;

    fn emitted(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DiffuseLight {
    pub emit: Color,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> DiffuseLight {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self) -> Color {
        self.emit
    }
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
        fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
            // Use Schlick's approximation for reflectance.
            let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            let r0 = r0.powi(2);
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit(ray_in.direction);

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);

        Some((scattered, self.albedo))
    }
}
