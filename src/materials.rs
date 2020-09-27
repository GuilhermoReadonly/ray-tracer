use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(
        ray_in: Ray,
        hit_record: HitRecord,
        color_attenuation: Color,
        scattered_ray: Ray,
    ) -> bool;
}
