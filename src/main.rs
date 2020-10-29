use ray_tracer::{self, RTError};
use std::time::Instant;
mod scenes;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    let samples_per_pixel = 100;

    // Create scene and empty image
    let (img, world, camera) = scenes::test_random_scene();

    // Render Image
    let now = Instant::now();
    let mut img = ray_tracer::Image::new(600, 800);//ray_tracer::create_img(img, world, samples_per_pixel, camera, 50);
    img.pixels = (0..800*600).map(|x| ray_tracer::Color::new_random()).collect();
    println!("Image generated in {} ms", now.elapsed().as_millis());

    // Write to .ppm file
    let now = Instant::now();
    ray_tracer::write_img_to_ppm("./target/img.ppm", img)?;
    println!("Image writed in {} ms", now.elapsed().as_millis());

    Ok(())
}
