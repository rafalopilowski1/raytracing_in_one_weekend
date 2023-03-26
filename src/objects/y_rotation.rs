use std::sync::Arc;

use crate::hittable::HitRecord;

use crate::ray::Ray;
use crate::{aabb::Aabb, vec3::Vec3};

use crate::Hittable;

pub struct YRotation<H: Hittable + ?Sized> {
    pub hittable: Arc<H>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_box: bool,
    pub bbox: Aabb,
}

impl<H: Hittable + ?Sized> YRotation<H> {
    pub fn new(hittable: Arc<H>, angle: f64) -> Arc<Self> {
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
        Arc::from(Self {
            hittable,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        })
    }
}

impl<H: Hittable + ?Sized> Hittable for YRotation<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x_r = self.cos_theta * ray.origin.x_r - self.sin_theta * ray.origin.z_b;
        origin.z_b = self.sin_theta * ray.origin.x_r + self.cos_theta * ray.origin.z_b;
        direction.x_r = self.cos_theta * ray.direction.x_r - self.sin_theta * ray.direction.z_b;
        direction.z_b = self.sin_theta * ray.direction.x_r + self.cos_theta * ray.direction.z_b;
        let rotated_ray = Ray::new(origin, direction, ray.time);
        if let Some(mut rec) = self.hittable.hit(&rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p.x_r = self.cos_theta * rec.p.x_r + self.sin_theta * rec.p.z_b;
            p.z_b = -self.sin_theta * rec.p.x_r + self.cos_theta * rec.p.z_b;
            normal.x_r = self.cos_theta * rec.normal.x_r + self.sin_theta * rec.normal.z_b;
            normal.z_b = -self.sin_theta * rec.normal.x_r + self.cos_theta * rec.normal.z_b;
            rec.p = p;
            let ray = &rotated_ray;
            rec.set_face_normal(ray, normal);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
}
