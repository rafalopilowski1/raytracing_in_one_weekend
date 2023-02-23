use std::sync::Arc;

use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

use crate::Hittable;

pub struct ConstantMedium<H: Hittable + ?Sized> {
    pub boundary: Arc<H>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl<H: Hittable + ?Sized> ConstantMedium<H> {
    pub fn new(boundary: Arc<H>, density: f64, phase_function: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        })
    }
}

impl<H: Hittable + ?Sized> Hittable for ConstantMedium<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, f64::MIN, f64::MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, f64::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = Vec3::length(ray.direction);
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let root = rec1.t + hit_distance / ray_length;
                Some(HitRecord {
                    p: ray.at(root),
                    t: root,
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    front_face: true,
                    material: Some(self.phase_function.clone()),
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
