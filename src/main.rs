use ray_tracer::{
    self,
    math::{Sphere, Vec3},
    Camera, Color, HittableList, Lambertian, Metal, RTError,
};
use std::time::Instant;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    let samples_per_pixel = 100;

    // Image
    let image_width: u32 = 800;
    let image_height: u32 = 450;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64; // = 16/9

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let camera = Camera::new(
        viewport_height,
        viewport_width,
        1.0,
        Vec3::new(0.0, 0.1, 0.0),
    );

    // Create world
    let mut world = HittableList::new();

    let material1: Lambertian = Lambertian::new(Color::new(0.8, 0.2, 0.2));
    let sphere1 = Box::new(Sphere::new(Vec3::new(0.75, 0.0, -1.0), 0.5, material1));
    world.add(sphere1);

    let material1: Lambertian = Lambertian::new(Color::new(0.0, 0.9, 0.2));
    let sphere1 = Box::new(Sphere::new(Vec3::new(-0.75, 0.0, -1.0), 0.5, material1));
    world.add(sphere1);

    let material1: Lambertian = Lambertian::new(Color::new(0.1, 0.1, 0.8));
    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 1.25, -2.0), 0.5, material1));
    world.add(sphere1);

    let material1: Metal = Metal::new(Color::new(0.6, 0.6, 0.6));
    let sphere1 = Box::new(Sphere::new(Vec3::new(-0.0, 0.0, -1.75), 0.5, material1));
    world.add(sphere1);

    let material2: Lambertian = Lambertian::new(Color::new(0.2, 0.2, 0.2));
    let sphere2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material2));
    world.add(sphere2);

    // Render Image
    let now = Instant::now();
    let img = ray_tracer::create_img(
        image_height,
        image_width,
        world,
        samples_per_pixel,
        camera,
        50,
    );
    println!("Image generated in {} ms", now.elapsed().as_millis());

    // Write to .ppm file
    let now = Instant::now();
    ray_tracer::write_img_to_ppm("./target/img.ppm", img)?;
    println!("Image writed in {} ms", now.elapsed().as_millis());

    Ok(())
}
