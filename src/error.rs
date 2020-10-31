use image::ImageError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::Error as IOError,
};

#[derive(Debug)]
pub enum RTError {
    IO(IOError),
    ImageRS(ImageError),
    EmptyImg,
    InconsistencySizePixels { h: u32, w: u32, nb_pixels: usize },
}

impl Display for RTError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            RTError::EmptyImg => write!(f, "The image is empty"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            RTError::IO(ref e) => e.fmt(f),
            RTError::ImageRS(ref e) => e.fmt(f),
            RTError::InconsistencySizePixels { h, w, nb_pixels } => write!(
                f,
                "The size {}*{} do not equals the nb of pixels {}",
                h, w, nb_pixels
            ),
        }
    }
}

impl Error for RTError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            RTError::IO(ref e) => Some(e),
            _ => None,
        }
    }
}
