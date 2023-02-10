mod camera;
mod hittable;
mod material;
mod objects;
mod ray;
mod vec3;

use crate::objects::Object;
use camera::Camera;

use hittable::{HitRecord, HittableList};

use image::ImageEncoder;
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use ray::Ray;

use std::{
    error::Error, f64::consts::PI, fs::File, io::BufWriter, mem::swap, sync::Arc, time::Instant,
};
use vec3::Vec3;

use crate::vec3::PixelResult;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub struct Random {
    pub rng: ThreadRng,
    pub uniform: Uniform<f64>,
}

impl Random {
    pub fn new(rng: ThreadRng, uniform: Uniform<f64>) -> Self {
        Self { rng, uniform }
    }
    fn random_float(&mut self, min: Option<f64>, max: Option<f64>) -> f64 {
        match (min.is_some(), max.is_some()) {
            (true, true) => self.rng.gen_range(min.unwrap()..max.unwrap()),
            _ => self.rng.sample(self.uniform),
        }
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI) / 180.0
}

#[allow(clippy::too_many_arguments)]

fn ray_color_iterative(
    ray: &mut Ray,
    objects: &[Object],
    rec: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    acc: &mut Vec3,
    rng: &mut Random,
    depth: &mut i8,
) {
    while HittableList::hit_anything(objects, ray, f64::MIN_POSITIVE, f64::MAX, rec) {
        if rec
            .material
            .unwrap()
            .scatter(rng, ray, rec, attenuation, scattered)
        {
            *acc = *acc * *attenuation;
            swap(ray, scattered);
            *depth = depth.checked_sub(1).unwrap_or(0);
            if *depth == 0 {
                break;
            }
        }
    }
}
const SAMPLES_PER_PIXEL: u32 = 400;
fn main() -> Result<(), Box<dyn Error>> {
    // World
    let rng = rand::thread_rng();
    let mut random = Random::new(rng, Uniform::new(0.0, 1.0));
    let world = Arc::new(HittableList::randon_scene(&mut random));

    // Camera
    let camera: Camera = Camera::default();

    // Image
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio) as u32;
    let mut imgbuf = image::RgbImage::new(image_width, image_height);

    // Render
    println!("Rendering...");

    let mut progress: u32 = 0;
    let pool = rayon::ThreadPoolBuilder::new()
        .thread_name(|idx| format!("Thread {}", idx))
        .build()?;
    let (tx, rx) = std::sync::mpsc::channel::<Option<PixelResult>>();
    let mut time1 = Instant::now();
    for h in 0..image_height {
        for w in 0..image_width {
            let world = world.clone();
            let tx = tx.clone();
            pool.spawn(move || {
                let pixel_color = work(
                    image_width - w,
                    image_width,
                    h,
                    image_height,
                    camera,
                    &world,
                );
                let pixel_result = Some(PixelResult::new(
                    pixel_color,
                    image_width - w - 1, // TODO: workaround to invert image; investigate why is it needed?
                    image_height - h - 1, // TODO: workaround to invert image; investigate why is it needed?
                ));
                tx.send(pixel_result).unwrap();
            });
        }
    }

    drop(tx);

    for (number, t) in rx.iter().enumerate() {
        if let Some(t) = t {
            imgbuf.put_pixel(t.x, t.y, t.color.into());
            let progress2 = number as u32 * 100 / (image_height * image_width);
            if progress2 > progress {
                let time2 = Instant::now();
                let eta = time2.duration_since(time1) * (100 - progress2);
                println!("{0}% - ETA: {1} sec.", progress2, eta.as_secs());
                progress = progress2;
                time1 = Instant::now();
            }
        } else {
            println!("Oops!");
        }
    }

    // Saving
    println!("Saving...");
    let file_ppm = File::create("image3.png")?;
    let buf_writer = BufWriter::new(file_ppm);
    let enc = image::codecs::png::PngEncoder::new(buf_writer);
    enc.write_image(&imgbuf, image_width, image_height, image::ColorType::Rgb8)?;

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
    let mut rng = Random::new(rand::thread_rng(), Uniform::new(0.0, 1.0));
    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
    let mut max_depth: i8 = 50;
    let objects = world.objects.as_slice();
    for _ in 0..SAMPLES_PER_PIXEL {
        let u = (width as f64 + rng.random_float(None, None)) / (image_width as f64 - 1.);
        let v = (height as f64 + rng.random_float(None, None)) / (image_height as f64 - 1.);
        let mut ray = Camera::get_ray(&mut rng, camera, u, v);
        let mut rec = HitRecord::default();
        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        // `acc` must be (1,1,1) for multiplication to work;
        let mut acc = Vec3::new(1., 1., 1.);
        ray_color_iterative(
            &mut ray,
            objects,
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
