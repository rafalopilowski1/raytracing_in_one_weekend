mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;

use std::{
    error::Error,
    f64::consts::PI,
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
};
use vec3::Vec3;

use crate::material::{Lamberian, Metal};

fn random_float<R: Rng + ?Sized>(rng: &mut R, min: Option<f64>, max: Option<f64>) -> f64 {
    match (min.is_some(), max.is_some()) {
        (true, true) => rng.gen_range(min.unwrap()..max.unwrap()),
        _ => rng.gen_range(0.0..1.0),
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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

fn ray_color(ray: &mut Ray, world: &mut HittableList, depth: u8) -> Vec3 {
    let mut rec = HitRecord::default();
    if depth == 0 {
        return Vec3::default();
    }
    if world.hit(*ray, f64::MIN_POSITIVE, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&mut scattered, world, depth - 1);
        }
        return Vec3::default();
    }
    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let material_ground = Arc::new(Lamberian::new(Vec3::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lamberian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));
    // World

    let mut world = HittableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0., -100.5, -1.),
            100.,
            material_ground,
        )),
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(1.0, 0., -1.), 0.5, material_left)),
        Box::new(Sphere::new(Vec3::new(-1.0, -0., -1.), 0.5, material_right)),
    ]);

    // Camera
    let camera: Camera = Camera::default();

    // Image
    const IMAGE_WIDTH: u16 = 400;
    let image_height: u16 = (IMAGE_WIDTH as f64 / camera.aspect_ratio) as u16;
    const SAMPLES_PER_PIXEL: u8 = 100;
    let mut MAX_DEPTH: u8 = 50;
    // Render

    let file_ppm = File::create("image.ppm")?;
    let mut buf_writer = BufWriter::new(file_ppm);
    buf_writer.write_all(format!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, image_height).as_bytes())?;
    let mut rng = rand::thread_rng();
    for h in (0..image_height).rev() {
        println!("Scanlines remaining: {0}", image_height - h);
        for w in (0..IMAGE_WIDTH).rev() {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (w as f64 + random_float(&mut rng, None, None)) / (IMAGE_WIDTH as f64 - 1.);
                let v =
                    (h as f64 + random_float(&mut rng, None, None)) / (image_height as f64 - 1.);
                let mut r = Camera::get_ray(&camera, u, v);
                //let mut rec = HitRecord::default();
                //let mut attenuation = Vec3::new(0., 0., 0.);
                //let mut scattered = Ray::default();
                pixel_color += ray_color(&mut r, &mut world, MAX_DEPTH);
            }

            write_color(&mut buf_writer, pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    println!("Done!");
    Ok(())
}

fn write_color(
    buf_writer: &mut BufWriter<File>,
    color: Vec3,
    samples_per_pixel: u8,
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
