use crate::{aabb::Aabb, hittable::HitRecord, material::Material, ray::Ray};
use std::f64::consts;

use std::sync::Arc;

use crate::vec3::Vec3;

use super::Hittable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let a: f64 = Vec3::length_squared(ray.direction);
        let oc = ray.origin - self.center;
        let half_b: f64 = Vec3::dot(oc, ray.direction);
        let c: f64 = Vec3::length_squared(oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.material = Some(self.material.clone());
        true
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
        let phi = p.z_b.atan2(p.x_r);
        let theta = p.y_g.asin();
        *u = 1. - (phi + consts::PI) / (2. * consts::PI);
        *v = (theta + consts::FRAC_PI_2) / consts::PI;
    }
}
