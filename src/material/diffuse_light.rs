use std::sync::Arc;

use crate::{hittable::HitRecord, random::Random, ray::Ray, texture::Texture, vec3::Vec3};

use super::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _rng: &mut Random<f64>,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.emit.color(_u, _v, _p)
    }
}
