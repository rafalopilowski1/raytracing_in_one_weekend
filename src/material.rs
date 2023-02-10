use std::sync::Arc;

use crate::{hittable::HitRecord, random::Random, texture::Texture, vec3::Vec3, Ray};
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub mod lamberian;
pub mod metal;
pub mod dielectric;
