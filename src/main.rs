use ray_tracer::{
    self,
    math::{Sphere, Vec3},
    Camera, HittableList, RTError,
};
use std::time::Instant;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    // Image
    let image_width: u32 = 400;
    let image_height: u32 = 225;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64; // = 16/9

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let camera = Camera::new(
        viewport_height,
        viewport_width,
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
    );

    // Create world
    let mut world = HittableList::new();

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(sphere1);
    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.add(sphere1);

    // Render Image
    let now = Instant::now();
    let img = ray_tracer::create_img(image_height, image_width, world, 100, camera, 50);
    println!("Image generated in {} ms", now.elapsed().as_millis());

    // Write to .ppm file
    let now = Instant::now();
    ray_tracer::write_img_to_ppm("./target/img.ppm", img)?;
    println!("Image writed in {} ms", now.elapsed().as_millis());

    Ok(())
}
