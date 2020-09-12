use crate::error::RTError;
use std::{fmt::Display, fs::File, io::Write};

#[derive(Debug, PartialEq)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub pixels: Vec<Color>,
    pub height: u32,
    pub width: u32,
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

impl Image {
    fn new(pixels: Vec<Color>, height: u32, width: u32) -> Self {
        Image {
            pixels,
            height,
            width,
        }
    }
}

pub fn create_img(img_height: u32, img_width: u32) -> Image {
    let capacity = (img_height * img_width) as usize;
    let mut img: Vec<Color> = Vec::with_capacity(capacity);

    for j in 0..img_height {
        for i in 0..img_width {
            // let progression = ((i+1)*(j+1)) as f64 / (img_height*img_width) as f64;
            // eprintln!("Progression: {}%", progression*100.0);

            let color = Color::new(
                (i as f64 / img_width as f64 * 256.0) as u32,
                (j as f64 / img_height as f64 * 256.0) as u32,
                (0.25 * 255.999) as u32,
            );

            img.push(color);
        }
    }

    Image::new(img, img_height, img_width)
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
    use test;

    #[test]
    fn create_img_empty() {
        let img = create_img(0, 0);
        let img_expected: Image = Image::new(vec![], 0, 0);

        assert_eq!(img_expected, img);
    }

    #[test]
    fn create_img_13_8() {
        let img = create_img(13, 8);
        let size_img_expected = 13 * 8;

        assert_eq!(size_img_expected, img.pixels.len());
        assert_eq!(13, img.height);
        assert_eq!(8, img.width);
    }

    #[test]
    fn write_bad_img() {
        let mut img = create_img(13, 8);
        img.height = 12;

        let result = write_img_to_ppm("./target/test.ppm", img);

        if let Err(RTError::InconsistencySizePixels { h, w, nb_pixels }) = result {
            assert_eq!(h, 12);
            assert_eq!(w, 8);
            assert_eq!(nb_pixels, 13 * 8);
        } else {
            panic!(
                "We should have a Err(RTError::InconsistencySizePixels) but we got: {:?}",
                result
            );
        }
    }

    #[test]
    fn write_img() {
        let img = create_img(13, 8);

        let result = write_img_to_ppm("./target/test.ppm", img);

        dbg!(&result);
        assert!(result.is_ok());
    }
}
