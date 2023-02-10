



use crate::{vec3::Vec3};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub mod checker_texture;
pub mod solid_color;

pub mod noise_texture;

pub mod image_texture;
