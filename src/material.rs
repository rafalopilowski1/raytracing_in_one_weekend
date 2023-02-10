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
pub mod lamberian;
pub mod metal;
pub mod isotropic;
