use ray_tracer::{self, RTError};
use std::time::Instant;
mod scenes;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    // Create scene, empty image and other parameters
    let (img, world, camera, samples_per_pixel, depth) = scenes::random_scene_with_lights();

    // Render Image
    let now = Instant::now();
    let img = ray_tracer::create_img(img, world, samples_per_pixel, camera, depth);
    let gen_time = now.elapsed().as_secs_f64();
    println!("Image generated in {} s", gen_time);

    // Write to file
    let now = Instant::now();
    ray_tracer::write_img_to_file("./target/img.jpg", &img)?;
    // Archive with parameters in file name
    ray_tracer::write_img_to_file(
        &format!(
            "./target/img-size_{}x{}-depth_{}-samples_{}-aperture_{}-focus_{}-fov_{:.2}-time_{:.3}.jpg",
            img.width, img.height, depth, samples_per_pixel, camera.aperture, camera.focus_dist, camera.vfov, gen_time
        ),
        &img,
    )?;
    println!("Image writed in {} s", now.elapsed().as_secs_f64());

    Ok(())
}
