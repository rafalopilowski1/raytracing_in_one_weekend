use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,

    pub time0: f64,
    pub time1: f64,

    pub radius: f64,

    pub material: Arc<dyn Material>,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let a: f64 = Vec3::length_squared(ray.direction);
        let oc = ray.origin - self.center;
        let half_b: f64 = Vec3::dot(oc, ray.direction);
        let c: f64 = Vec3::length_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
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

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
        let a: f64 = Vec3::length_squared(ray.direction);
        let half_b: f64 = Vec3::dot(oc, ray.direction);
        let c: f64 = Vec3::length_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center(ray.time)) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(self.material.clone());
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = Aabb::surrounding_box(box0, box1);
        true
    }
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let phi = f64::atan2(p.z_b, p.x_r);
        let theta = f64::asin(p.y_g);
        *u = 1. - (phi + std::f64::consts::PI) / (2. * std::f64::consts::PI);
        *v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
    }
}
