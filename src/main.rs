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

use std::{error::Error, f64::consts::PI, fs::File, io::BufWriter};
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
                if *depth == 0 {
                    break;
                } else {
                    *depth -= 1;
                    continue;
                }
            }
            break;
        }
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
        let t = 0.5 * (1.0 + unit_direction.y_g);
        *acc = *acc * (Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t);
        break;
    }
}
const SAMPLES_PER_PIXEL: u32 = 500;
fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut rng = rand::thread_rng();
    let world = HittableList::randon_scene(&mut rng);

    // Camera
    let camera: Camera = Camera::default();

    // Image
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio) as u32;
    let mut imgbuf = image::RgbImage::new(image_width, image_height);

    // Render
    println!("Rendering...");
    let mut progress: u32 = 0;
    for (w, h, pixel) in imgbuf.enumerate_pixels_mut() {
        let pixel_color = work(w, image_width, h, image_height, camera, &world);
        *pixel = pixel_color.into();
        let progress2 = (w + (image_width * h)) * 100 / (image_height * image_width);
        if progress2 > progress {
            println!("{}%", progress2);
            progress = progress2;
        }
    }

    // Saving
    println!("Saving...");
    let file_ppm = File::create("image2.png")?;
    let buf_writer = BufWriter::new(file_ppm);
    let enc = image::png::PngEncoder::new(buf_writer);
    enc.encode(&imgbuf, image_width, image_height, image::ColorType::Rgb8)?;

    println!("Done!");
    Ok(())
}
fn work(
    width: u32,
    image_width: u32,
    height: u32,
    image_height: u32,
    camera: Camera,
    world: &HittableList,
) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
    let mut max_depth: u8 = 50;
    for _ in 0..SAMPLES_PER_PIXEL {
        let u = (width as f64 + random_float(&mut rng, None, None)) / (image_width as f64 - 1.);
        let v = (height as f64 + random_float(&mut rng, None, None)) / (image_height as f64 - 1.);
        let mut ray = Camera::get_ray(&mut rng, camera, u, v);
        let mut rec = HitRecord::default();
        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        // `acc` must be (1,1,1) for multiplication to work;
        let mut acc = Vec3::new(1., 1., 1.);
        ray_color_iterative(
            &mut ray,
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
