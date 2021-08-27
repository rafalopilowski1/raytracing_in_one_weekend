use crate::{hittable::HitRecord, vec3::Vec3, Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
pub struct Lamberian {
    albedo: Vec3,
}
impl Lamberian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

pub struct Metal {
    albedo: Vec3,
}
impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lamberian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(&mut rng);
        if Vec3::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction, rec.normal) > 0.
    }
}
