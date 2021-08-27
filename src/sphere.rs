use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a: f64 = Vec3::length_squared(ray.direction);
        let half_b: f64 = Vec3::dot(oc, ray.direction);
        let c: f64 = Vec3::length_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        true
    }
}
