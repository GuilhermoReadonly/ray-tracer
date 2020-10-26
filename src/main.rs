use ray_tracer::{
    self,
    math::{Sphere, Vec3},
    Camera, Color, HittableList, Lambertian, Metal, RTError, Dielectric,
};
use std::time::Instant;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    let samples_per_pixel = 100;

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

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    //let material_center2 = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
    let material_left = Dielectric::new(Color::new(0.9, 0.9, 0.9), 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let sphere_ground = Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    let sphere_center = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    //let sphere_center2 = Box::new(Sphere::new(Vec3::new(0.0, 0.75, -2.0), 0.5, material_center2));
    let sphere_left = Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let sphere_right = Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    world.add(sphere_ground);
    world.add(sphere_center);
    //world.add(sphere_center2);
    world.add(sphere_left);
    world.add(sphere_right);

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
