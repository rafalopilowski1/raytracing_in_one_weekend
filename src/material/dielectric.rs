use crate::{hittable::HitRecord, random::Random, vec3::Vec3, Ray};
use std::sync::Arc;

use crate::Material;

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub(crate) ir: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(ray_in.direction);

        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract
            || Dielectric::reflactance(cos_theta, refraction_ratio) > rng.random(None, None)
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::reflact(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction, ray_in.time);
        true
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Arc<Self> {
        Arc::from(Self { ir })
    }

    pub fn reflactance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0.powi(2);
        r0 * (1. - r0) * (1. - cosine).powi(5)
    }
}
