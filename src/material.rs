use rand::RngCore;

use crate::{hittable::HitRecord, vec3::Vec3, Ray};
#[derive(Clone, Copy)]
pub enum Material {
    Lamberian(Lamberian),
    Metal(Metal),
    Dielectric(Dielectric),
}
impl Material {
    pub fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lamberian(lamberian) => {
                lamberian.scatter(rng, ray_in, rec, attenuation, scattered)
            }
            Material::Metal(metal) => metal.scatter(rng, ray_in, rec, attenuation, scattered),
            Material::Dielectric(dielectric) => {
                dielectric.scatter(rng, ray_in, rec, attenuation, scattered)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lamberian {
    albedo: Vec3,
}
impl Lamberian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        if Vec3::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere(rng) * self.fuzz,
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction, rec.normal) > 0.
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
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
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
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

        let direction: Vec3;
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract
            || Dielectric::reflactance(cos_theta, refraction_ratio)
                > crate::random_float(rng, None, None)
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::reflact(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
