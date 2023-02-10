use std::sync::Arc;

use crate::{texture::Texture, vec3::Vec3};

use super::Material;

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        rng: &mut crate::random::Random<f64>,
        ray_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        *scattered = crate::ray::Ray::new(rec.p, Vec3::random_in_unit_sphere(rng), ray_in.time);
        *attenuation = self.albedo.color(rec.u, rec.v, rec.p);
        true
    }
}
