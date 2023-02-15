use crate::{hittable::HitRecord, objects::Aabb, ray::Ray, vec3::Vec3, Hittable, Material};

use std::sync::Arc;

pub struct xy_rect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl xy_rect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        })
    }
}

impl Hittable for xy_rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z_b) / ray.direction.z_b;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x_r + t * ray.direction.x_r;
        let y = ray.origin.y_g + t * ray.direction.y_g;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.material = Some(self.mp.clone());
        rec.p = ray.at(t);
        rec.set_face_normal(ray, Vec3::new(0.0, 0.0, 1.0));
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}
pub struct xz_rect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl xz_rect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        })
    }
}

impl Hittable for xz_rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y_g) / ray.direction.y_g;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x_r + t * ray.direction.x_r;
        let z = ray.origin.z_b + t * ray.direction.z_b;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.material = Some(self.mp.clone());
        rec.p = ray.at(t);
        rec.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}
pub struct yz_rect {
    pub mp: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl yz_rect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self {
            mp,
            y0,
            y1,
            z0,
            z1,
            k,
        })
    }
}
impl Hittable for yz_rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x_r) / ray.direction.x_r;
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.origin.y_g + t * ray.direction.y_g;
        let z = ray.origin.z_b + t * ray.direction.z_b;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.material = Some(self.mp.clone());
        rec.p = ray.at(t);
        rec.set_face_normal(ray, Vec3::new(1.0, 0.0, 0.0));
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
