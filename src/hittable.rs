use crate::{
    material::{Lamberian, Material},
    ray::Ray,
    vec3::Vec3,
};
#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,

    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                *rec = temp_rec;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
