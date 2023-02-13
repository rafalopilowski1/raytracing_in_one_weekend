mod aabb;
mod bvh_node;
mod camera;
mod hittable;
mod material;
mod objects;
mod perlin;
mod random;
mod ray;
mod texture;
mod vec3;

use camera::Camera;

use hittable::{HitRecord, HittableList};
use image::ImageEncoder;
use objects::Hittable;
use rand::distributions::Uniform;
use random::Random;
use ray::Ray;
use rayon::iter::ParallelIterator;
use std::{
    error::Error, f64::consts::PI, fs::File, io::BufWriter, mem::swap, sync::Arc, time::Instant,
};
use vec3::PixelResult;
use vec3::Vec3;

use mimalloc::MiMalloc;
use rayon::iter::IntoParallelIterator;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI) / 180.0
}

#[allow(clippy::too_many_arguments)]

fn ray_color_iterative(
    ray: &mut Ray,
    hittable_list: &HittableList,
    rec: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    background: &mut Vec3,
    emitted: &mut Vec3,
    rng: &mut Random<f64>,
    depth: i8,
) -> Vec3 {
    let mut acc = Vec3::new(1., 1., 1.);
    let mut depth_count = depth;
    loop {
        if hittable_list.hit(ray, f64::MIN_POSITIVE, f64::MAX, rec) {
            *emitted = rec.material.as_ref().unwrap().emitted(rec.u, rec.v, rec.p);
            if rec
                .material
                .as_ref()
                .unwrap()
                .scatter(rng, ray, rec, attenuation, scattered)
            {
                acc = acc * *attenuation + *emitted;
                swap(ray, scattered);
                depth_count = depth_count.checked_sub(1).unwrap_or(0);
                if depth_count <= 0 {
                    break;
                }
            } else {
                acc = acc * *emitted;
                break;
            }
        } else {
            acc = acc * *background;
            break;
        }
    }
    acc
}
const SAMPLES_PER_PIXEL: u32 = 10000;
const MAX_DEPTH: i8 = 50;
const IMAGE_WIDTH: u32 = 1080;
fn main() -> Result<(), Box<dyn Error>> {
    // World
    let choice = 7;
    let rng = rand::thread_rng();
    let mut random = Random::new(rng, Uniform::new(0.0, 1.0));
    let world = match choice {
        0 => Arc::new(HittableList::randon_scene(&mut random)),
        1 => Arc::new(HittableList::two_spheres(&mut random)),
        2 => Arc::new(HittableList::two_perlin_spheres(&mut random)),
        3 => Arc::new(HittableList::earth()),
        4 => Arc::new(HittableList::simple_light(&mut random)),
        5 => Arc::new(HittableList::cornell_box()),
        6 => Arc::new(HittableList::cornell_smoke()),
        7 => Arc::new(HittableList::final_scene(&mut random)),
        _ => Arc::new(HittableList::randon_scene(&mut random)),
    };

    // Camera
    let camera: Camera = Camera::new(
        Vec3::new(478., 278., -600.),
        Vec3::new(278., 278., 0.),
        Vec3::new(0., 1., 0.),
        40.0,
        1.,
        0.1,
        0.,
        1.,
    );

    // Image
    let image_height: u32 = (IMAGE_WIDTH as f64 / camera.aspect_ratio) as u32;
    let mut img_buf = image::RgbImage::new(IMAGE_WIDTH, image_height);

    // Render
    println!("Rendering...");

    let mut progress: u32 = 0;
    let mut time1 = Instant::now();
    for h in 0..image_height {
        for w in 0..IMAGE_WIDTH {
            let pixel_color = work(
                IMAGE_WIDTH - w,
                IMAGE_WIDTH,
                h,
                image_height,
                camera,
                &world,
                &mut random,
            );
            let pixel_result = PixelResult::new(
                pixel_color,
                IMAGE_WIDTH - w - 1, // TODO: workaround to invert image; investigate why is it needed?
                image_height - h - 1, // TODO: workaround to invert image; investigate why is it needed?
            );
            img_buf.put_pixel(pixel_result.x, pixel_result.y, pixel_result.color.into());

            let progress2 = (w + (IMAGE_WIDTH * h) * 100) / (image_height * IMAGE_WIDTH);
            if progress2 > progress {
                let time2 = Instant::now();
                let eta = time2.duration_since(time1) * (100 - progress2);
                if eta.as_secs() > 3600 {
                    println!(
                        "{0}% - ETA: {1} h. {2} min. {3} sec.",
                        progress2,
                        eta.as_secs() / 3600,
                        (eta.as_secs() % 3600) / 60,
                        eta.as_secs() % 60
                    );
                } else if eta.as_secs() > 60 {
                    println!(
                        "{0}% - ETA: {1} min. {2} sec.",
                        progress2,
                        eta.as_secs() / 60,
                        eta.as_secs() % 60
                    );
                } else {
                    println!("{0}% - ETA: {1} sec.", progress2, eta.as_secs() % 60);
                }

                progress = progress2;
                time1 = Instant::now();
            }
        }
    }

    // Saving
    println!("Saving...");
    let file_ppm = File::create("image3.png")?;
    let buf_writer = BufWriter::new(file_ppm);
    let enc = image::codecs::png::PngEncoder::new(buf_writer);
    enc.write_image(&img_buf, IMAGE_WIDTH, image_height, image::ColorType::Rgb8)?;

    println!("Done!");
    Ok(())
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
            let mut rng = Random::new(rand::thread_rng(), Uniform::new(0.0, 1.0));
            let mut ray = Camera::get_ray(&mut rng, camera, u, v);
            let mut rec = HitRecord::default();
            let mut attenuation = Vec3::default();
            let mut scattered = Ray::default();
            let mut background = Vec3::new(0., 0., 0.);
            let mut emitted = Vec3::default();

            ray_color_iterative(
                &mut ray,
                world,
                &mut rec,
                &mut attenuation,
                &mut scattered,
                &mut background,
                &mut emitted,
                &mut rng,
                MAX_DEPTH,
            )
        })
        .reduce(Vec3::default, |a, b| a + b)
}
