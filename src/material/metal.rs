use crate::hittable::HitRecord;

use crate::Ray;

use crate::random::Random;

use super::Material;

use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Metal {
    pub(crate) albedo: Vec3,
    pub(crate) fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut Random<f64>,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere(rng) * self.fuzz,
            ray_in.time,
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction, rec.normal) > 0.
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}
