use std::sync::Arc;

use crate::{aabb::Aabb, vec3::Vec3};

use super::Hittable;

pub struct YRotation {
    pub hittable: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_box: bool,
    pub bbox: Aabb,
}

impl YRotation {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let has_box = hittable.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Vec3::new(f64::MIN, f64::MIN, f64::MIN);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x_r + (1 - i) as f64 * bbox.min.x_r;
                    let y = j as f64 * bbox.max.y_g + (1 - j) as f64 * bbox.min.y_g;
                    let z = k as f64 * bbox.max.z_b + (1 - k) as f64 * bbox.min.z_b;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        bbox = Aabb::new(min, max);
        Self {
            hittable,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }
}

impl Hittable for YRotation {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x_r = self.cos_theta * ray.origin.x_r - self.sin_theta * ray.origin.z_b;
        origin.z_b = self.sin_theta * ray.origin.x_r + self.cos_theta * ray.origin.z_b;
        direction.x_r = self.cos_theta * ray.direction.x_r - self.sin_theta * ray.direction.z_b;
        direction.z_b = self.sin_theta * ray.direction.x_r + self.cos_theta * ray.direction.z_b;
        let rotated_ray = crate::ray::Ray::new(origin, direction, ray.time);
        if self.hittable.hit(&rotated_ray, t_min, t_max, rec) {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p.x_r = self.cos_theta * rec.p.x_r + self.sin_theta * rec.p.z_b;
            p.z_b = -self.sin_theta * rec.p.x_r + self.cos_theta * rec.p.z_b;
            normal.x_r = self.cos_theta * rec.normal.x_r + self.sin_theta * rec.normal.z_b;
            normal.z_b = -self.sin_theta * rec.normal.x_r + self.cos_theta * rec.normal.z_b;
            rec.p = p;
            rec.set_face_normal(&rotated_ray, normal);
            true
        } else {
            false
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
}
