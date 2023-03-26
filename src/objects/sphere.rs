use crate::{aabb::Aabb, hittable::HitRecord, material::Material, ray::Ray};
use std::f64::consts;

use std::sync::Arc;

use crate::vec3::Vec3;

use crate::Hittable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a: f64 = Vec3::length_squared(ray.direction);
        let oc = ray.origin - self.center;
        let half_b: f64 = Vec3::dot(oc, ray.direction);
        let c: f64 = Vec3::length_squared(oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            material: Some(self.material.clone()),
            ..Default::default()
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self {
            center,
            radius,
            material,
        })
    }
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        *u = 0.5 + f64::atan2(-p.z_b, p.x_r) * 0.5 * consts::FRAC_1_PI;
        *v = 0.5 + f64::asin(p.y_g) * consts::FRAC_1_PI;
    }
}
