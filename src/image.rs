use crate::{clamp, math::Vec3, Camera, RTError, Ray, World};
use image::ImageBuffer;
use rand::Rng;
use std::{collections::HashMap, ops, time::Instant};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub vec: Vec3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            vec: Vec3::new(r, g, b),
        }
    }

    pub fn new_with_vec(vec: Vec3) -> Self {
        Color { vec }
    }

    pub fn new_random() -> Self {
        Color::new_with_vec(Vec3::new_random(0.0, 1.0))
    }

    pub fn r(&self) -> f64 {
        self.vec.x
    }
    pub fn g(&self) -> f64 {
        self.vec.y
    }
    pub fn b(&self) -> f64 {
        self.vec.z
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new_with_vec(&self.vec * &other.vec)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new_with_vec(&self.vec + &other.vec)
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub pixels: HashMap<(u32, u32), Color>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = HashMap::with_capacity((height * width) as usize);
        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn add_pixel(&mut self, width: u32, height: u32, color: Color) {
        self.pixels.insert((width, height), color);
    }

    pub fn get_color_pixel(&self, width: u32, height: u32) -> Color {
        let mut color = self.pixels.get(&(width, height));
        let default = Color::new(0.0, 0.0, 0.0);
        **color.get_or_insert(&default)
    }
}

pub fn create_img<F>(
    mut img: Image,
    world: World<F>,
    samples_per_pixel: u32,
    camera: Camera,
    depth: u32,
) -> Image
where
    F: Fn(&Ray) -> Color,
{
    let mut rng = rand::thread_rng();

    let total_rays_to_trace: u64 = img.height as u64 * img.width as u64 * samples_per_pixel as u64;
    let mut ray_traced: u64 = 0;

    let mut timer = Instant::now();

    for h in (0..img.height).rev() {
        for w in 0..img.width {
            // let progression = ((i+1)*(j+1)) as f64 / (img_height*img_width) as f64;
            // eprintln!("Progression: {}%", progression*100.0);
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (w as f64 + rng.gen_range(0.0..1.0)) / (img.width - 1) as f64;
                let v = (h as f64 + rng.gen_range(0.0..1.0)) / (img.height - 1) as f64;
                let ray: Ray = camera.get_ray(u, v);

                let ray_color = ray.ray_color(&world, depth);
                pixel_color.vec = pixel_color.vec + ray_color.vec;

                ray_traced = ray_traced + 1;
                if timer.elapsed().as_secs_f64() >= 1.0 {
                    let percent = ray_traced as f64 / total_rays_to_trace as f64 * 100.0;
                    println!(
                        "{:.2}% done, {} over {}",
                        percent, ray_traced, total_rays_to_trace
                    );
                    timer = Instant::now();
                }
            }
            pixel_color.vec = pixel_color.vec / samples_per_pixel as f64;
            img.add_pixel(w, h, pixel_color);
        }
    }

    img
}

pub fn write_img_to_file(path: &str, img: &Image) -> Result<(), RTError> {
    if img.height * img.width != img.pixels.len() as u32 {
        return Err(RTError::InconsistencySizePixels {
            h: img.height,
            w: img.width,
            nb_pixels: img.pixels.len(),
        });
    };

    let img_to_write = ImageBuffer::from_fn(img.width, img.height, |w, h| {
        let color = img.get_color_pixel(w, img.height - h);
        let r = (256.0 * clamp(color.r().sqrt(), 0.0, 0.999)) as u8;
        let g = (256.0 * clamp(color.g().sqrt(), 0.0, 0.999)) as u8;
        let b = (256.0 * clamp(color.b().sqrt(), 0.0, 0.999)) as u8;

        image::Rgb([r, g, b])
    });

    img_to_write.save(path).map_err(|e| RTError::ImageRS(e))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::math::Sphere;
//     use test;

//     fn create_rand_size() -> (u32, u32) {
//         let mut rng = rand::thread_rng();

//         (rng.gen_range(0, 50), rng.gen_range(0, 50))
//     }

//     fn create_rand_img() -> (Image, u32, u32) {
//         let (h, w) = create_rand_size();
//         let aspect_ratio: f64 = h as f64 / w as f64;
//         let viewport_height = 2.0;
//         let viewport_width = aspect_ratio * viewport_height;

//         let camera = Camera::new(
//             viewport_height,
//             viewport_width,
//             1.0,
//             Vec3::new(0.0, 0.0, 0.0),
//         );

//         let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
//         let mut world = HittableList::new();
//         world.add(Box::new(sphere));

//         (create_img(h, w, world, 1, camera, 10), h, w)
//     }

//     #[test]
//     fn create_img_test() {
//         let (img, h, w) = create_rand_img();
//         let size_img_expected = h * w;

//         assert_eq!(size_img_expected, img.pixels.len() as u32);
//         assert_eq!(h, img.height);
//         assert_eq!(w, img.width);
//     }

//     #[test]
//     fn write_bad_img() {
//         let (mut img, h_origin, w_origin) = create_rand_img();
//         let h_modified = h_origin + 1;
//         let w_modified = w_origin + 1;
//         img.height = h_modified;
//         img.width = w_modified;

//         let result = write_img_to_ppm("./target/test.ppm", img);

//         if let Err(RTError::InconsistencySizePixels { h, w, nb_pixels }) = result {
//             assert_eq!(h, h_modified);
//             assert_eq!(w, w_modified);
//             assert_eq!(nb_pixels as u32, h_origin * w_origin);
//         } else {
//             panic!(
//                 "We should have a Err(RTError::InconsistencySizePixels) but we got: {:?}",
//                 result
//             );
//         }
//     }

//     #[test]
//     fn write_img() {
//         let (img, _, _) = create_rand_img();

//         let result = write_img_to_ppm("./target/test.ppm", img);

//         dbg!(&result);
//         assert!(result.is_ok());
//     }
// }
