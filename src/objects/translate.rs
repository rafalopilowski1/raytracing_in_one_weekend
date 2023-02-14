use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::Hittable;

pub struct Translate {
    pub hittable: Arc<dyn Hittable>,
    pub offset: Vec3,
}
impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, offset: Vec3) -> Arc<Self> {
        Arc::from(Self { hittable, offset })
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if self.hittable.hit(&moved, t_min, t_max, rec) {
            rec.p += self.offset;
            true
        } else {
            false
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.hittable.bounding_box(time0, time1, output_box) {
            *output_box = Aabb::new(output_box.min + self.offset, output_box.max + self.offset);
            true
        } else {
            false
        }
    }
}
