use crate::{error::RTError, math::Vec3};
use std::{fmt::Display, fs::File, io::Write};

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
        write!(
            f,
            "{} {} {}",
            (self.r() * 256.0) as u32,
            (self.g() * 256.0) as u32,
            (self.b() * 256.0) as u32
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

pub fn create_img(img_height: u32, img_width: u32) -> Image {
    let capacity = (img_height * img_width) as usize;
    let mut pixels: Vec<Color> = Vec::with_capacity(capacity);

    for j in 0..img_height {
        for i in 0..img_width {
            // let progression = ((i+1)*(j+1)) as f64 / (img_height*img_width) as f64;
            // eprintln!("Progression: {}%", progression*100.0);

            let color = Color::new(
                i as f64 / img_width as f64,
                j as f64 / img_height as f64,
                0.25,
            );

            pixels.push(color);
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
    use rand::prelude::*;
    use test;

    fn create_rand_size() -> (u32, u32) {
        let mut rng = rand::thread_rng();

        (rng.gen_range(0, 50), rng.gen_range(0, 50))
    }

    fn create_rand_img() -> (Image, u32, u32) {
        let (h, w) = create_rand_size();
        (create_img(h, w), h, w)
    }

    #[test]
    fn create_img_empty() {
        let img = create_img(0, 0);
        let img_expected: Image = Image::new(vec![], 0, 0);

        assert_eq!(img_expected, img);
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
