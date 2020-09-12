use ray_tracer::{error::RTError, image};
use std::time::Instant;

fn main() -> Result<(), RTError> {
    println!("Starting...");

    let now = Instant::now();
    let img = image::create_img(256, 256);
    println!("Image generated in {} ns", now.elapsed().as_nanos());

    let now = Instant::now();
    image::write_img_to_ppm("./test.ppm", img)?;
    println!("Image writed in {} ns", now.elapsed().as_nanos());

    Ok(())
}
