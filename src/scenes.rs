use rand::Rng;
use ray_tracer::{
    self,
    math::{Sphere, Vec3, TAU},
    Camera, Color, Dielectric, HittableList, Image, Lambertian, Metal,
};

#[allow(unused)]
pub fn test_defocus_scene() -> (Image, HittableList, Camera) {
    let aspect_ratio = 16.0 / 9.0;

    // World
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(Color::new(0.9, 0.9, 0.9), 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    let sphere_ground = Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    let sphere_center = Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
    world.add(sphere_ground);
    world.add(sphere_center);
    //world.add(sphere_center2);
    world.add(sphere_left);
    world.add(sphere_right);

    // Camera
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 2.0;
    let focus_dist = (lookfrom - lookat).length();
    let vfov = 20.0 / 360.0 * TAU;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // Image
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let img = Image::new(image_height, image_width);

    (img, world, camera)
}

#[allow(unused)]
pub fn test_random_scene() -> (Image, HittableList, Camera) {
    let aspect_ratio = 3.0 / 2.0;

    // Image
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let img = Image::new(image_width, image_height);

    // World
    let mut world = HittableList::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new_boxed(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<(f64)>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<(f64)>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo: Color = Color::new_random() * Color::new_random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new_boxed(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = Color::new_random();
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new_boxed(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material =
                        Dielectric::new(Color::new_random(), rng.gen_range(0.0, 3.0));
                    world.add(Sphere::new_boxed(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material1 = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
    world.add(Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0 / 360.0 * TAU;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let aperture = 0.1;
    let focus_dist = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    (img, world, camera)
}
