use ray_tracer::{image, RTError};
use std::time::Instant;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    // Image
    let image_width: u32 = 400;
    let image_height: u32 = 225;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Render Image
    let now = Instant::now();
    let img = image::create_img(
        image_height,
        image_width,
        viewport_height,
        viewport_width,
        focal_length,
    );
    println!("Image generated in {} ns", now.elapsed().as_nanos());

    // Write to .ppm file
    let now = Instant::now();
    image::write_img_to_ppm("./target/img.ppm", img)?;
    println!("Image writed in {} ns", now.elapsed().as_nanos());

    Ok(())
}
