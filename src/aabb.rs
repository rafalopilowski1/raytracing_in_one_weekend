use crate::{hittable::HitRecord, objects::Hittable, ray::Ray, vec3::Vec3};

#[derive(Default, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let small = Vec3::new(
            box0.min.x_r.min(box1.min.x_r),
            box0.min.y_g.min(box1.min.y_g),
            box0.min.z_b.min(box1.min.z_b),
        );
        let big = Vec3::new(
            box0.max.x_r.max(box1.max.x_r),
            box0.max.y_g.max(box1.max.y_g),
            box0.max.z_b.max(box1.max.z_b),
        );
        Self::new(small, big)
    }
}
impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rec: &mut HitRecord) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        let inv_d = Vec3::new(
            1.0 / ray.direction.x_r,
            1.0 / ray.direction.y_g,
            1.0 / ray.direction.z_b,
        );
        let mut t0 = (self.min - ray.origin) * inv_d;
        let mut t1 = (self.max - ray.origin) * inv_d;
        for a in 0..=2 {
            if inv_d[a] < 0.0 {
                std::mem::swap(&mut t0[a], &mut t1[a]);
            }
            t_min = t_min.max(t0[a]);
            t_max = t_max.min(t1[a]);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = *self;
        true
    }
}
