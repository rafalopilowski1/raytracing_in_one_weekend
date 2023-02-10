use std::{path::Path, sync::Arc};

use crate::{
    aabb::Aabb,
    objects::{moving_sphere::MovingSphere, sphere::Sphere, Hittable},
    random::{self, Random},
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color::SolidColor,
    },
};

use crate::{
    material::{dielectric::Dielectric, lamberian::Lamberian, metal::Metal, Material},
    ray::Ray,
    vec3::Vec3,
};
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
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
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub type HittableThreadSafe = Arc<dyn Hittable>;

pub struct HittableList {
    pub objects: Vec<HittableThreadSafe>,
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|object| {
            if object.hit(ray, t_min, closest_so_far, rec) {
                closest_so_far = rec.t;
                hit_anything = true;
            }
        });

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box: Aabb = Default::default();
        let mut first_box = true;
        for object in self.objects.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                Aabb::surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }
        true
    }
}
impl HittableList {
    pub fn new(objects: Vec<HittableThreadSafe>) -> Self {
        Self { objects }
    }

    pub fn randon_scene(rng: &mut random::Random<f64>) -> HittableList {
        let mut world = HittableList::new(vec![]);

        let checker = Arc::new(CheckerTexture::new(
            Arc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
            Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
        ));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Arc::new(Lamberian::new(checker)),
        )));
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.random(None, None);
                let center = Vec3::new(
                    a as f64 + 0.9 * rng.random(None, None),
                    0.2,
                    b as f64 + rng.random(None, None),
                );

                if Vec3::length(center - Vec3::new(4., 0.2, 0.)) > 0.9 {
                    let sphere_material: Option<Arc<dyn Material>>;
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random(rng, None, None) * Vec3::random(rng, None, None);
                        sphere_material =
                            Some(Arc::new(Lamberian::new(Arc::new(SolidColor::new(albedo)))));
                        let center2 = center + Vec3::new(0., rng.random(Some(0.), Some(0.5)), 0.);
                        world.objects.push(Arc::new(MovingSphere::new(
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
                        let fuzz = rng.random(Some(0.), Some(0.5));
                        sphere_material = Some(Arc::new(Metal::new(albedo, fuzz)));
                    } else {
                        // glass
                        sphere_material = Some(Arc::new(Dielectric::new(1.5)));
                    }
                    world.objects.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.unwrap(),
                    )));
                }
            }
        }

        let material1 = Arc::new(Dielectric::new(1.5));
        let material2 = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.4, 0.2, 0.1,
        )))));
        let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

        world
            .objects
            .push(Arc::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material1)));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(-4.0, 1., 0.),
            1.0,
            material2,
        )));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(4.0, 1., 0.),
            1.0,
            material3,
        )));
        world
    }

    pub fn two_spheres(_rng: &mut random::Random<f64>) -> HittableList {
        let mut world = HittableList::new(vec![]);
        let checker = Arc::new(CheckerTexture::new(
            Arc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
            Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
        ));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., -10., 0.),
            10.,
            Arc::new(Lamberian::new(checker.clone())),
        )));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., 10., 0.),
            10.,
            Arc::new(Lamberian::new(checker)),
        )));
        world
    }

    pub(crate) fn two_perlin_spheres(rng: &mut Random<f64>) -> HittableList {
        let mut world = HittableList::new(vec![]);
        let pertext = Arc::new(NoiseTexture::new(rng, 4.0));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Arc::new(Lamberian::new(pertext.clone())),
        )));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., 2., 0.),
            2.,
            Arc::new(Lamberian::new(pertext)),
        )));
        world
    }

    pub(crate) fn earth() -> HittableList {
        let mut world = HittableList::new(vec![]);
        let earth_texture = Arc::new(ImageTexture::new(Path::new(
            "/home/rafal_opilowski/Code/raytracing_in_one_weekend/earthmap.jpg",
        )));
        let earth_surface = Arc::new(Lamberian::new(earth_texture));
        let globe = Arc::new(Sphere::new(Vec3::new(0., 0., 0.), 2., earth_surface));
        world.objects.push(globe);
        world
    }

    pub(crate) fn simple_light() -> HittableList {
        todo!()
    }

    pub(crate) fn cornell_box() -> HittableList {
        todo!()
    }

    pub(crate) fn cornell_smoke() -> HittableList {
        todo!()
    }

    pub(crate) fn final_scene(_random: &mut random::Random<f64>) -> HittableList {
        todo!()
    }
}
