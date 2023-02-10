use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}
pub mod moving_sphere;
pub mod sphere;
