mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;

use hittable::{HitRecord, Hittable, HittableList};
use rand::{Rng, RngCore};
use ray::Ray;

use std::{
    error::Error,
    f64::consts::PI,
    fs::File,
    io::{BufWriter, Write},
};
use vec3::Vec3;

fn random_float<R: Rng + ?Sized>(rng: &mut R, min: Option<f64>, max: Option<f64>) -> f64 {
    match (min.is_some(), max.is_some()) {
        (true, true) => rng.gen_range(min.unwrap()..max.unwrap()),
        _ => rng.gen_range(0.0..1.0),
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI) / 180.0
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
#[allow(clippy::too_many_arguments)]
fn ray_color_iterative(
    ray: &mut Ray,
    world: &HittableList,
    rec: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    acc: &mut Vec3,
    rng: &mut dyn RngCore,
    depth: &mut u8,
) {
    loop {
        if world.hit(*ray, f64::MIN_POSITIVE, f64::INFINITY, rec) {
            if rec.material.scatter(rng, ray, rec, attenuation, scattered) {
                *acc = *acc * *attenuation;
                *ray = *scattered;
                *depth -= 1;
                if *depth == 0 {
                    break;
                } else {
                    continue;
                }
            }
            break;
        }
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
        let t = 0.5 * (1.0 + unit_direction.y);
        *acc = *acc * (Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t);
        break;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut rng = rand::thread_rng();
    let world = HittableList::randon_scene(&mut rng);

    // Camera
    let camera: Camera = Camera::default();

    // Image
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / camera.aspect_ratio) as u16;
    let samples_per_pixel: u16 = 100;

    // Render
    let file_ppm = File::create("image.ppm")?;
    let mut buf_writer = BufWriter::new(file_ppm);
    buf_writer.write_all(format!("P3\n{0} {1}\n255\n", image_width, image_height).as_bytes())?;

    for h in (0..image_height).rev() {
        println!("Scanning: {}", image_height - h);
        for w in 0..image_width {
            let pixel_color = work(
                samples_per_pixel,
                w,
                image_width,
                h,
                image_height,
                camera,
                &world,
            );

            write_color(&mut buf_writer, pixel_color, samples_per_pixel).unwrap();
        }
    }
    println!("Done!");
    Ok(())
}
fn work(
    samples_per_pixel: u16,
    w: u16,
    image_width: u16,
    h: u16,
    image_height: u16,
    camera: Camera,
    world: &HittableList,
) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
    let mut max_depth: u8 = 50;
    for _ in 0..samples_per_pixel {
        let u = (w as f64 + random_float(&mut rng, None, None)) / (image_width as f64 - 1.);
        let v = (h as f64 + random_float(&mut rng, None, None)) / (image_height as f64 - 1.);
        let mut r = Camera::get_ray(&mut rng, camera, u, v);
        let mut rec = HitRecord::default();
        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        // `acc` must be (1,1,1) for multiplication to work;
        let mut acc = Vec3::new(1., 1., 1.);
        ray_color_iterative(
            &mut r,
            world,
            &mut rec,
            &mut attenuation,
            &mut scattered,
            &mut acc,
            &mut rng,
            &mut max_depth,
        );
        pixel_color += acc;
    }
    pixel_color
}

fn write_color(
    buf_writer: &mut BufWriter<File>,
    color: Vec3,
    samples_per_pixel: u16,
) -> Result<usize, std::io::Error> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let ir: u16 = (clamp(r, 0.0, 0.999) * 256.) as u16;
    let ig: u16 = (clamp(g, 0.0, 0.999) * 256.) as u16;
    let ib: u16 = (clamp(b, 0.0, 0.999) * 256.) as u16;
    buf_writer.write(format!("{0} {1} {2}\n", ir, ig, ib).as_bytes())
}
