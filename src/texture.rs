use std::{path::Path, sync::Arc};

use image::{GenericImageView, Rgb32FImage};

use crate::{perlin::Perlin, random::Random, vec3::Vec3};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}
impl SolidColor {
    pub fn new(color_value: Vec3) -> Self {
        Self { color_value }
    }
}
impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}
impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x_r).sin() * (10. * p.y_g).sin() * (10. * p.z_b).sin();
        if sines < 0. {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(rng: &mut Random<f64>, scale: f64) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z_b + 10.0 * self.noise.turb(&p, 7)))
    }
}

pub struct ImageTexture {
    data: Option<Rgb32FImage>,
    width: usize,
    height: usize,
}
impl ImageTexture {
    pub fn new(path: &Path) -> Self {
        let img = image::open(path).ok();
        if let Some(img) = img {
            let (width, height) = img.dimensions();
            let data = img.into_rgb32f();
            Self {
                data: Some(data),
                width: width as usize,
                height: height as usize,
            }
        } else {
            Self {
                data: None,
                width: 0,
                height: 0,
            }
        }
    }
}
impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        if let Some(data) = &self.data {
            let u_work = u.clamp(0., 1.);
            let v_work = 1.0 - v.clamp(0., 1.);

            let mut i = (u_work * self.width as f64) as usize;
            let mut j = (v_work * self.height as f64) as usize;

            if i >= self.width {
                i = self.width - 1;
            }
            if j >= self.height {
                j = self.height - 1;
            }

            // let color_scale = 1.0 / 255.0;
            let pixel = data.get_pixel(i as u32, j as u32);
            Vec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
        } else {
            Vec3::new(255.0, 0.0, 0.0)
        }
    }
}
