use crate::{
    objects::{MovingSphere, Object},
    random_float,
};
use rand::RngCore;

use crate::{
    material::{Dielectric, Lamberian, Material, Metal},
    objects::Sphere,
    ray::Ray,
    vec3::Vec3,
};
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
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
            material: None,
            t: 0.0,
            front_face: false,
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Object>,
}

impl HittableList {
    pub fn new(objects: Vec<Object>) -> Self {
        Self { objects }
    }
    pub fn hit_anything(
        objects: &[Object],
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in objects {
            if object.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
    pub fn randon_scene(rng: &mut dyn RngCore) -> HittableList {
        let mut world = HittableList::new(vec![]);

        let ground_material = Material::Lamberian(Lamberian::new(Vec3::new(0.5, 0.5, 0.5)));
        world.objects.push(Object::Sphere(Sphere::new(
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
                    let mut sphere_material: Option<Material> = None;
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random(rng, None, None) * Vec3::random(rng, None, None);
                        sphere_material = Some(Material::Lamberian(Lamberian::new(albedo)));
                        let center2 =
                            center + Vec3::new(0., random_float(rng, Some(0.), Some(0.5)), 0.);
                        world.objects.push(Object::MovingSphere(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material.unwrap(),
                        )));
                        continue;
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random(rng, Some(0.5), Some(1.));
                        let fuzz = crate::random_float(rng, Some(0.), Some(0.5));
                        sphere_material = Some(Material::Metal(Metal::new(albedo, fuzz)));
                    } else {
                        // glass
                        sphere_material = Some(Material::Dielectric(Dielectric::new(1.5)));
                    }
                    world.objects.push(Object::Sphere(Sphere::new(
                        center,
                        0.2,
                        sphere_material.unwrap(),
                    )));
                }
            }
        }

        let material1 = Material::Dielectric(Dielectric::new(1.5));
        let material2 = Material::Lamberian(Lamberian::new(Vec3::new(0.4, 0.2, 0.1)));
        let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

        world.objects.push(Object::Sphere(Sphere::new(
            Vec3::new(0., 1., 0.),
            1.0,
            material1,
        )));
        world.objects.push(Object::Sphere(Sphere::new(
            Vec3::new(-4.0, 1., 0.),
            1.0,
            material2,
        )));
        world.objects.push(Object::Sphere(Sphere::new(
            Vec3::new(4.0, 1., 0.),
            1.0,
            material3,
        )));
        world
    }
}
