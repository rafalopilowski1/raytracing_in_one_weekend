use std::sync::Arc;

use crate::{hittable::HitRecord, random::Random, vec3::Vec3, Ray};
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lamberian;
pub mod metal;

impl<T: Material + ?Sized> Material for Option<&Arc<T>> {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        if let Some(material) = self {
            material.scatter(rng, ray_in, rec, attenuation, scattered)
        } else {
            false
        }
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        if let Some(material) = self {
            material.emitted(_u, _v, _p)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
