use crate::{hittable::HitRecord, random::Random, texture::Texture, vec3::Vec3, Ray};

use crate::Material;

use std::sync::Arc;

#[derive(Clone)]
pub struct Lamberian {
    pub(crate) albedo: Arc<dyn Texture>,
}

impl Material for Lamberian {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        if Vec3::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        *attenuation = self.albedo.color(rec.u, rec.v, rec.p);
        true
    }
}

impl Lamberian {
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::from(Self { albedo })
    }
}
