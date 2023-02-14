use crate::vec3::Vec3;

use super::Texture;

use image::{self, GenericImageView};

use std::path::Path;

use image::Rgb32FImage;

pub struct ImageTexture {
    pub(crate) data: Option<Rgb32FImage>,
    pub(crate) width: usize,
    pub(crate) height: usize,
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
            let pixel = data.get_pixel(i as u32, j as u32);
            Vec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
        } else {
            Vec3::new(255.0, 0.0, 0.0)
        }
    }
}
