mod aabb;
mod bvh_node;
mod camera;
mod hittable;
mod image_env_builder;
mod material;
mod objects;
mod perlin;
mod random;
mod ray;
mod texture;
mod vec3;

use camera::Camera;

use hittable::{HitRecord, HittableList};
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use material::Material;
use objects::Hittable;
use rand::distributions::Uniform;
use random::Random;
use ray::Ray;
use rayon::{iter::IntoParallelIterator, iter::ParallelIterator};
use std::{error::Error, fs::File, io::BufWriter, mem::swap, sync::Arc, time::Instant};
use vec3::Vec3;

use mimalloc::MiMalloc;

use crate::image_env_builder::ImageEnvBuilder;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SAMPLES_PER_PIXEL: u32 = 1000;
const MAX_DEPTH: i8 = 50;
const IMAGE_WIDTH: u32 = 1280;
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let choice = 7;
    let mut rng = rand::thread_rng();
    let mut random = Random::new(&mut rng, Uniform::new(0.0, 1.0));
    // Camera
    let (camera, world) = ImageEnvBuilder::build(choice, &mut random);

    // Image
    let mut img_buf = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render
    println!("Rendering...");

    let mut progress: u32 = 0;
    let mut time1 = Instant::now();
    (0..IMAGE_WIDTH * IMAGE_HEIGHT).for_each(|i| {
        let mut random = Random::new(&mut rng, Uniform::new(0.0, 1.0));
        let w = i % IMAGE_WIDTH;
        let h = i / IMAGE_WIDTH;
        // TODO: workaround to invert image; investigate why is it needed?
        let pixel_color = work(
            IMAGE_WIDTH - w,
            IMAGE_WIDTH,
            h,
            IMAGE_HEIGHT,
            camera,
            &world,
            &mut random,
        );
        // TODO: workaround to invert image; investigate why is it needed?
        img_buf.put_pixel(
            IMAGE_WIDTH - w - 1,
            IMAGE_HEIGHT - h - 1,
            pixel_color.into(),
        );
        display_progress(&mut progress, &mut time1, h, w);
    });

    // Saving
    println!("Saving...");
    let file_ppm = File::create(format!("./render{}.png", choice))?;
    let buf_writer = BufWriter::new(file_ppm);
    let enc = PngEncoder::new(buf_writer);
    enc.write_image(&img_buf, IMAGE_WIDTH, IMAGE_HEIGHT, ColorType::Rgb8)?;

    println!("Done!");
    Ok(())
}

fn display_progress(progress: &mut u32, time1: &mut Instant, h: u32, w: u32) {
    let progress2 = (w + (IMAGE_WIDTH * h) * 100) / (IMAGE_HEIGHT * IMAGE_WIDTH);
    if progress2 > *progress {
        let time2 = Instant::now();
        let duration_since = time2.duration_since(*time1);
        let eta = duration_since * (100 - progress2);
        if eta.as_secs() > 3600 {
            println!(
                "{0}% - ETA: {1} h. {2} min. {3} sec. ({4:.2} rays/sec)",
                progress2,
                eta.as_secs() / 3600,
                (eta.as_secs() % 3600) / 60,
                eta.as_secs() % 60,
                SAMPLES_PER_PIXEL as f64 / duration_since.as_secs_f64()
                    * (IMAGE_WIDTH as f64 * IMAGE_HEIGHT as f64 * 0.01)
            );
        } else if eta.as_secs() > 60 {
            println!(
                "{0}% - ETA: {1} min. {2} sec. ({3:.2} rays/sec)",
                progress2,
                eta.as_secs() / 60,
                eta.as_secs() % 60,
                SAMPLES_PER_PIXEL as f64 / duration_since.as_secs_f64()
                    * (IMAGE_WIDTH as f64 * IMAGE_HEIGHT as f64 * 0.01)
            );
        } else {
            println!(
                "{0}% - ETA: {1} sec. ({2:.2} rays/sec)",
                progress2,
                eta.as_secs() % 60,
                SAMPLES_PER_PIXEL as f64 / duration_since.as_secs_f64()
                    * (IMAGE_WIDTH as f64 * IMAGE_HEIGHT as f64 * 0.01),
            );
        }

        *progress = progress2;
        *time1 = Instant::now();
    }
}

fn work(
    width: u32,
    image_width: u32,
    height: u32,
    image_height: u32,
    camera: Camera,
    world: &Arc<HittableList>,
    rng: &mut Random<f64>,
) -> Vec3 {
    let u = (width as f64 + rng.random(None, None)) / (image_width as f64 - 1.);
    let v = (height as f64 + rng.random(None, None)) / (image_height as f64 - 1.);
    (0..SAMPLES_PER_PIXEL)
        .into_par_iter()
        .map(|_| {
            let mut rand = rand::thread_rng();
            let mut rng = Random::new(&mut rand, Uniform::new(0.0, 1.0));
            let mut ray = Camera::get_ray(&mut rng, camera, u, v);
            let mut background = Vec3::new(0., 0., 0.);
            ray_color_iterative(&mut ray, world, &mut background, &mut rng, MAX_DEPTH)
        })
        .reduce(Vec3::default, |a, b| a + b)
}

fn ray_color_iterative(
    ray: &mut Ray,
    hittable_list: &HittableList,
    background: &mut Vec3,
    rng: &mut Random<f64>,
    depth: i8,
) -> Vec3 {
    let mut acc = Vec3::new(1., 1., 1.);
    let mut depth_count = depth;
    let mut rec = HitRecord::default();
    let mut attenuation = Vec3::default();
    let mut scattered = Ray::default();
    let mut emitted;
    loop {
        if hittable_list.hit(ray, f64::MIN_POSITIVE, f64::MAX, &mut rec) {
            emitted = rec.material.as_ref().emitted(rec.u, rec.v, rec.p);
            if rec
                .material
                .as_ref()
                .scatter(rng, ray, &rec, &mut attenuation, &mut scattered)
            {
                acc = acc * attenuation + emitted;
                swap(ray, &mut scattered);
                depth_count = depth_count.checked_sub(1).unwrap_or(0);
                if depth_count == 0 {
                    break;
                }
            } else {
                acc = acc * emitted;
                break;
            }
        } else {
            acc = acc * *background;
            break;
        }
    }
    acc
}
