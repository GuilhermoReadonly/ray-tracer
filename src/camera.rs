use crate::{math::Vec3, Ray};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
        origin: Vec3,
    ) -> Camera {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            aspect_ratio: viewport_width / viewport_height,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}

#[test]
fn test_camera() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let viewport_height: f64 = rng.gen_range(10.0, 100.0);
    let viewport_width: f64 = rng.gen_range(10.0, 100.0);
    let focal_length: f64 = rng.gen_range(0.5, 10.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(viewport_height, viewport_width, focal_length, origin);

    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    assert_eq!(camera.viewport_height, viewport_height);
    assert_eq!(camera.viewport_width, viewport_width);
    assert_eq!(camera.focal_length, focal_length);
    assert_eq!(camera.origin, origin);

    assert_eq!(camera.aspect_ratio, viewport_width / viewport_height);
    assert_eq!(camera.horizontal, horizontal);
    assert_eq!(camera.vertical, vertical);
    assert_eq!(
        camera.lower_left_corner,
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length)
    );

    let u: f64 = rng.gen_range(0.0, 1.0);
    let v: f64 = rng.gen_range(0.0, 1.0);
    let ray = camera.get_ray(u, v);

    assert_eq!(ray.origin, camera.origin);
    assert_eq!(
        ray.direction,
        camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin
    );
}
