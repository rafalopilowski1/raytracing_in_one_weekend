use std::sync::Arc;

use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

use super::Hittable;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f64,
        phase_function: Arc<dyn Material>,
    ) -> Arc<Self> {
        Arc::from(Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self.boundary.hit(ray, f64::MIN, f64::MAX, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(ray, rec1.t + 0.0001, f64::MAX, &mut rec2) {
            return false;
        }
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = Vec3::length(ray.direction);
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = Some(self.phase_function.clone());
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
