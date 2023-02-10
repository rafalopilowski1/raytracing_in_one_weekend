use std::{path::Path, sync::Arc};

use rand::distributions::Uniform;

use crate::{
    aabb::Aabb,
    bvh_node::BvhNode,
    material::{diffuse_light::DiffuseLight, isotropic::Isotropic},
    objects::{
        box_render::BoxRender, constant_medium::ConstantMedium, moving_sphere::MovingSphere,
        sphere::Sphere, translate::Translate, xy_rect::xy_rect, xz_rect::xz_rect,
        y_rotation::YRotation, yz_rect::yz_rect, Hittable,
    },
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

    pub(crate) fn simple_light(rng: &mut Random<f64>) -> HittableList {
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

        let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3::new(
            4., 4., 4.,
        )))));
        world
            .objects
            .push(Arc::new(xy_rect::new(3., 5., 1., 3., -2., difflight)));

        world
    }

    pub(crate) fn cornell_box() -> HittableList {
        let mut world = HittableList::new(vec![]);

        let red = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.65, 0.05, 0.05,
        )))));
        let white = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.73, 0.73, 0.73,
        )))));
        let green = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.12, 0.45, 0.15,
        )))));
        let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3::new(
            15., 15., 15.,
        )))));

        world
            .objects
            .push(Arc::new(yz_rect::new(0., 555., 0., 555., 555., green)));
        world
            .objects
            .push(Arc::new(yz_rect::new(0., 555., 0., 555., 0., red)));
        world
            .objects
            .push(Arc::new(xz_rect::new(213., 343., 227., 332., 554., light)));
        world.objects.push(Arc::new(xz_rect::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            white.clone(),
        )));
        world.objects.push(Arc::new(xz_rect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.objects.push(Arc::new(xy_rect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let mut box1: Arc<dyn Hittable> = Arc::new(BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 330., 165.),
            white.clone(),
        ));
        box1 = Arc::new(YRotation::new(box1, 15.));
        box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
        box1 = Arc::new(ConstantMedium::new(
            box1,
            0.01,
            Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
                0., 0., 0.,
            ))))),
        ));
        world.objects.push(box1);

        let mut box2: Arc<dyn Hittable> = Arc::new(BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 165., 165.),
            white,
        ));
        box2 = Arc::new(YRotation::new(box2, -18.));
        box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
        box2 = Arc::new(ConstantMedium::new(
            box2,
            0.01,
            Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
                1., 1., 1.,
            ))))),
        ));
        world.objects.push(box2);
        world
    }

    pub(crate) fn cornell_smoke() -> HittableList {
        let mut world = HittableList::new(vec![]);

        let red = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.65, 0.05, 0.05,
        )))));
        let white = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.73, 0.73, 0.73,
        )))));
        let green = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.12, 0.45, 0.15,
        )))));
        let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3::new(
            7., 7., 7.,
        )))));

        world
            .objects
            .push(Arc::new(yz_rect::new(0., 555., 0., 555., 555., green)));
        world
            .objects
            .push(Arc::new(yz_rect::new(0., 555., 0., 555., 0., red)));
        world
            .objects
            .push(Arc::new(xz_rect::new(113., 443., 127., 432., 554., light)));
        world.objects.push(Arc::new(xz_rect::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            white.clone(),
        )));
        world.objects.push(Arc::new(xz_rect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));
        world.objects.push(Arc::new(xy_rect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
        )));

        let mut box1: Arc<dyn Hittable> = Arc::new(BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 330., 165.),
            white.clone(),
        ));
        box1 = Arc::new(YRotation::new(box1, 15.));
        box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
        box1 = Arc::new(ConstantMedium::new(
            box1,
            0.01,
            Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
                0., 0., 0.,
            ))))),
        ));
        world.objects.push(box1);

        let mut box2: Arc<dyn Hittable> = Arc::new(BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 165., 165.),
            white,
        ));
        box2 = Arc::new(YRotation::new(box2, -18.));
        box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
        box2 = Arc::new(ConstantMedium::new(
            box2,
            0.01,
            Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
                1., 1., 1.,
            ))))),
        ));
        world.objects.push(box2);
        world
    }

    pub(crate) fn final_scene(random: &mut random::Random<f64>) -> HittableList {
        let _rng = Random::new(rand::thread_rng(), Uniform::new(0usize, 3));
        let mut boxes1 = HittableList::new(vec![]);
        let ground = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.48, 0.83, 0.53,
        )))));

        const BOXES_PER_SIDE: usize = 20;
        for i in 0..BOXES_PER_SIDE {
            for j in 0..BOXES_PER_SIDE {
                let w = 100.;
                let x0 = -1000. + i as f64 * w;
                let z0 = -1000. + j as f64 * w;
                let y0 = 0.;
                let x1 = x0 + w;
                let y1 = random.random(Some(1.), Some(101.));
                let z1 = z0 + w;

                boxes1.objects.push(Arc::new(BoxRender::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }

        let mut world = HittableList::new(vec![]);
        world
            .objects
            .push(Arc::new(BvhNode::new(&mut boxes1.objects, 0., 1.)));

        let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3::new(
            7., 7., 7.,
        )))));
        world
            .objects
            .push(Arc::new(xz_rect::new(123., 423., 147., 412., 554., light)));

        let center1 = Vec3::new(400., 400., 200.);
        let center2 = center1 + Vec3::new(30., 0., 0.);
        let moving_sphere_material = Arc::new(Lamberian::new(Arc::new(SolidColor::new(
            Vec3::new(0.7, 0.3, 0.1),
        ))));
        world.objects.push(Arc::new(MovingSphere::new(
            center1,
            center2,
            0.,
            1.,
            50.,
            moving_sphere_material,
        )));

        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(260., 150., 45.),
            50.,
            Arc::new(Dielectric::new(1.5)),
        )));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(0., 150., 145.),
            50.,
            Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.)),
        )));

        let boundary = Arc::new(Sphere::new(
            Vec3::new(360., 150., 145.),
            70.,
            Arc::new(Dielectric::new(1.5)),
        ));
        world.objects.push(boundary.clone());
        world.objects.push(Arc::new(ConstantMedium::new(
            boundary,
            0.2,
            Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
                0.2, 0.4, 0.9,
            ))))),
        )));
        let boundary2 = Arc::new(Sphere::new(
            Vec3::new(0., 0., 0.),
            5000.,
            Arc::new(Dielectric::new(1.5)),
        ));
        world.objects.push(Arc::new(ConstantMedium::new(
            boundary2,
            0.0001,
            Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
                1., 1., 1.,
            ))))),
        )));

        let emat = Arc::new(Lamberian::new(Arc::new(ImageTexture::new(Path::new(
            "earthmap.jpg",
        )))));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(400., 200., 400.),
            100.,
            emat,
        )));
        let pertext = Arc::new(NoiseTexture::new(random, 0.1));
        world.objects.push(Arc::new(Sphere::new(
            Vec3::new(220., 280., 300.),
            80.,
            Arc::new(Lamberian::new(pertext)),
        )));

        let mut boxes2 = HittableList::new(vec![]);
        let white = Arc::new(Lamberian::new(Arc::new(SolidColor::new(Vec3::new(
            0.73, 0.73, 0.73,
        )))));
        const NS: usize = 1000;
        for _j in 0..NS {
            boxes2.objects.push(Arc::new(Sphere::new(
                Vec3::new(
                    165. * random.random(Some(0.), Some(1.)),
                    165. * random.random(Some(0.), Some(1.)),
                    165. * random.random(Some(0.), Some(1.)),
                ),
                10.,
                white.clone(),
            )));
        }

        world.objects.push(Arc::new(Translate::new(
            Arc::new(YRotation::new(
                Arc::new(BvhNode::new(&mut boxes2.objects, 0., 1.)),
                15.,
            )),
            Vec3::new(-100., 270., 395.),
        )));
        world
    }
}
