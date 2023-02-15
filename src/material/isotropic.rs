use std::sync::Arc;

use crate::{hittable::HitRecord, random::Random, ray::Ray, texture::Texture, vec3::Vec3};

use crate::Material;

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::from(Self { albedo })
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(rng), ray_in.time);
        *attenuation = self.albedo.color(rec.u, rec.v, rec.p);
        true
    }
}
