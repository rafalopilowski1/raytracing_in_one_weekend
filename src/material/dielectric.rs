use crate::vec3::Vec3;

use crate::hittable::HitRecord;

use crate::Ray;

use crate::random::Random;

use super::Material;

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

        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

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
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    pub fn reflactance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 * (1. - r0) * f64::powf(1. - cosine, 5.0)
    }
}
