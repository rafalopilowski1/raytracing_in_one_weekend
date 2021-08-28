use std::rc::Rc;

use rand::RngCore;

use crate::{
    material::{Dielectric, Lamberian, Material, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
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
            material: Rc::new(Lamberian::new(Vec3::default())),
            t: 0.0,
            front_face: false,
        }
    }
}
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }
    pub fn randon_scene(rng: &mut dyn RngCore) -> HittableList {
        let mut world = HittableList::new(vec![]);
        let ground_material = Rc::new(Lamberian::new(Vec3::new(0.5, 0.5, 0.5)));
        world.objects.push(Rc::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            ground_material,
        )));
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = crate::random_float(rng, None, None);
                let center = Vec3::new(
                    a as f64 + 0.9 * crate::random_float(rng, None, None),
                    0.2,
                    b as f64 + crate::random_float(rng, None, None),
                );

                if Vec3::length(center - Vec3::new(4., 0.2, 0.)) > 0.9 {
                    let mut sphere_material: Option<Rc<dyn Material>> = None;
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random(rng, None, None) * Vec3::random(rng, None, None);
                        sphere_material = Some(Rc::new(Lamberian::new(albedo)));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random(rng, Some(0.5), Some(1.));
                        let fuzz = crate::random_float(rng, Some(0.), Some(0.5));
                        sphere_material = Some(Rc::new(Metal::new(albedo, fuzz)));
                    } else {
                        // glass
                        sphere_material = Some(Rc::new(Dielectric::new(1.5)));
                    }
                    world
                        .objects
                        .push(Rc::new(Sphere::new(center, 0.2, sphere_material.unwrap())));
                }
            }
        }

        let material1 = Rc::new(Dielectric::new(1.5));
        let material2 = Rc::new(Lamberian::new(Vec3::new(0.4, 0.2, 0.1)));
        let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

        world
            .objects
            .push(Rc::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material1)));
        world.objects.push(Rc::new(Sphere::new(
            Vec3::new(-4.0, 1., 0.),
            1.0,
            material2,
        )));
        world
            .objects
            .push(Rc::new(Sphere::new(Vec3::new(4.0, 1., 0.), 1.0, material3)));
        world
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
                *rec = temp_rec.clone();
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
