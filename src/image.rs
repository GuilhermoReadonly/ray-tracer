use crate::{clamp, math::Vec3, Camera, HittableList, RTError, Ray};
use rand::Rng;
use std::{fmt::Display, fs::File, io::Write};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub vec: Vec3,
    pub samples_per_pixel: u32,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, samples_per_pixel: u32) -> Self {
        Color {
            vec: Vec3::new(r, g, b),
            samples_per_pixel,
        }
    }

    pub fn new_with_vec(vec: Vec3, samples_per_pixel: u32) -> Self {
        Color {
            vec,
            samples_per_pixel,
        }
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = 1.0 / self.samples_per_pixel as f64;

        let r = (self.r() * scale).sqrt();
        let g = (self.g() * scale).sqrt();
        let b = (self.b() * scale).sqrt();

        write!(
            f,
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as u32,
            (256.0 * clamp(g, 0.0, 0.999)) as u32,
            (256.0 * clamp(b, 0.0, 0.999)) as u32,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub pixels: Vec<Color>,
    pub height: u32,
    pub width: u32,
}

impl Image {
    fn new(pixels: Vec<Color>, height: u32, width: u32) -> Self {
        Image {
            pixels,
            height,
            width,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        for p in self.pixels.iter() {
            write!(f, "{}\n", p)?;
        }
        Ok(())
    }
}

pub fn create_img(
    img_height: u32,
    img_width: u32,
    world: HittableList,
    samples_per_pixel: u32,
    camera: Camera,
    depth: u32,
) -> Image {
    let mut pixels: Vec<Color> = Vec::with_capacity((img_height * img_width) as usize);

    let mut rng = rand::thread_rng();

    for j in (0..img_height).rev() {
        for i in 0..img_width {
            // let progression = ((i+1)*(j+1)) as f64 / (img_height*img_width) as f64;
            // eprintln!("Progression: {}%", progression*100.0);
            let mut pixel_color = Color::new(0.0, 0.0, 0.0, samples_per_pixel);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (img_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / (img_height - 1) as f64;
                let ray: Ray = camera.get_ray(u, v);

                let ray_color = ray.ray_color(&world, samples_per_pixel, depth);
                pixel_color.vec = pixel_color.vec + ray_color.vec;
            }

            pixels.push(pixel_color);
        }
    }

    Image::new(pixels, img_height, img_width)
}

pub fn write_img_to_ppm(path: &str, img: Image) -> Result<(), RTError> {
    if img.height * img.width != img.pixels.len() as u32 {
        return Err(RTError::InconsistencySizePixels {
            h: img.height,
            w: img.width,
            nb_pixels: img.pixels.len(),
        });
    };
    let mut file = File::create(&path).map_err(|e| RTError::IO(e))?;

    file.write_all((&img.to_string()).as_bytes())
        .map_err(|e| RTError::IO(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Sphere;
    use test;

    fn create_rand_size() -> (u32, u32) {
        let mut rng = rand::thread_rng();

        (rng.gen_range(0, 50), rng.gen_range(0, 50))
    }

    fn create_rand_img() -> (Image, u32, u32) {
        let (h, w) = create_rand_size();
        let aspect_ratio: f64 = h as f64 / w as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let camera = Camera::new(
            viewport_height,
            viewport_width,
            1.0,
            Vec3::new(0.0, 0.0, 0.0),
        );

        let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let mut world = HittableList::new();
        world.add(Box::new(sphere));

        (create_img(h, w, world, 1, camera, 10), h, w)
    }

    #[test]
    fn create_img_test() {
        let (img, h, w) = create_rand_img();
        let size_img_expected = h * w;

        assert_eq!(size_img_expected, img.pixels.len() as u32);
        assert_eq!(h, img.height);
        assert_eq!(w, img.width);
    }

    #[test]
    fn write_bad_img() {
        let (mut img, h_origin, w_origin) = create_rand_img();
        let h_modified = h_origin + 1;
        let w_modified = w_origin + 1;
        img.height = h_modified;
        img.width = w_modified;

        let result = write_img_to_ppm("./target/test.ppm", img);

        if let Err(RTError::InconsistencySizePixels { h, w, nb_pixels }) = result {
            assert_eq!(h, h_modified);
            assert_eq!(w, w_modified);
            assert_eq!(nb_pixels as u32, h_origin * w_origin);
        } else {
            panic!(
                "We should have a Err(RTError::InconsistencySizePixels) but we got: {:?}",
                result
            );
        }
    }

    #[test]
    fn write_img() {
        let (img, _, _) = create_rand_img();

        let result = write_img_to_ppm("./target/test.ppm", img);

        dbg!(&result);
        assert!(result.is_ok());
    }
}
