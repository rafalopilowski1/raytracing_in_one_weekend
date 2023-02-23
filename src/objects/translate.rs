use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, ray::Ray, vec3::Vec3};

use crate::Hittable;

pub struct Translate<H: Hittable + ?Sized> {
    pub hittable: Arc<H>,
    pub offset: Vec3,
}
impl<H: Hittable + ?Sized> Translate<H> {
    pub fn new(hittable: Arc<H>, offset: Vec3) -> Arc<Self> {
        Arc::from(Self { hittable, offset })
    }
}

impl<H: Hittable + ?Sized> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(mut rec) = self.hittable.hit(&moved, t_min, t_max) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
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
