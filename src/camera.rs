use crate::{math::Vec3, Ray};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::new_random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

// #[test]
// fn test_camera() {
//     use rand::Rng;

//     let mut rng = rand::thread_rng();

//     let viewport_height: f64 = rng.gen_range(10.0, 100.0);
//     let viewport_width: f64 = rng.gen_range(10.0, 100.0);
//     let focal_length: f64 = rng.gen_range(0.5, 10.0);
//     let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

//     let camera = Camera::new(viewport_height, viewport_width, focal_length, origin);

//     let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
//     let vertical = Vec3::new(0.0, viewport_height, 0.0);

//     assert_eq!(camera.viewport_height, viewport_height);
//     assert_eq!(camera.viewport_width, viewport_width);
//     assert_eq!(camera.focal_length, focal_length);
//     assert_eq!(camera.lookfrom, origin);

//     assert_eq!(camera.aspect_ratio, viewport_width / viewport_height);
//     assert_eq!(camera.horizontal, horizontal);
//     assert_eq!(camera.vertical, vertical);
//     assert_eq!(
//         camera.lower_left_corner,
//         origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length)
//     );

//     let u: f64 = rng.gen_range(0.0, 1.0);
//     let v: f64 = rng.gen_range(0.0, 1.0);
//     let ray = camera.get_ray(u, v);

//     assert_eq!(ray.origin, camera.origin);
//     assert_eq!(
//         ray.direction,
//         camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin
//     );
// }
