use crate::{
    bvh_node::BvhNode,
    camera::Camera,
    hittable::HittableList,
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic,
        lamberian::Lamberian, metal::Metal, Material,
    },
    objects::{
        box_render::BoxRender,
        constant_medium::ConstantMedium,
        moving_sphere::MovingSphere,
        rect::{xy_rect, xz_rect, yz_rect},
        sphere::Sphere,
        translate::Translate,
        y_rotation::YRotation,
    },
    random::Random,
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color::SolidColor,
    },
    vec3::Vec3,
    Hittable, ASPECT_RATIO,
};
use std::{path::Path, sync::Arc};

pub struct ImageEnvBuilder {}

impl ImageEnvBuilder {
    pub fn build(choice: u32, random: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        match choice {
            0 => Self::randon_scene(random),
            1 => Self::two_spheres(random),
            2 => Self::two_perlin_spheres(random),
            3 => Self::earth(),
            4 => Self::simple_light(random),
            5 => Self::cornell_box(),
            6 => Self::cornell_smoke(),
            7 => Self::final_scene(random),
            _ => Self::randon_scene(random),
        }
    }
    pub fn randon_scene(rng: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);

        let checker = CheckerTexture::new(
            SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
            SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
        );
        world.objects.push(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            DiffuseLight::new(checker),
        ));
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
                        sphere_material = Some(Lamberian::new(SolidColor::new(albedo)));
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
                        sphere_material = Some(Metal::new(albedo, fuzz));
                    } else {
                        // glass
                        sphere_material = Some(Dielectric::new(1.5));
                    }
                    world
                        .objects
                        .push(Sphere::new(center, 0.2, sphere_material.unwrap()));
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        let material2 = Lamberian::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1)));
        let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);

        world
            .objects
            .push(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material1));
        world
            .objects
            .push(Sphere::new(Vec3::new(-4.0, 1., 0.), 1.0, material2));
        world
            .objects
            .push(Sphere::new(Vec3::new(4.0, 1., 0.), 1.0, material3));
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            10.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn two_spheres(_rng: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);
        let checker = CheckerTexture::new(
            SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
            SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
        );
        world.objects.push(Sphere::new(
            Vec3::new(0., -10., 0.),
            10.,
            Lamberian::new(checker.clone()),
        ));
        world.objects.push(Sphere::new(
            Vec3::new(0., 10., 0.),
            10.,
            DiffuseLight::new(checker),
        ));
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn two_perlin_spheres(rng: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);
        let pertext = Arc::new(NoiseTexture::new(rng, 4.0));
        world.objects.push(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            DiffuseLight::new(pertext.clone()),
        ));
        world.objects.push(Sphere::new(
            Vec3::new(0., 2., 0.),
            2.,
            Lamberian::new(pertext),
        ));
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn earth() -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);
        let earth_texture = Arc::new(ImageTexture::new(Path::new("earthmap.jpg")));
        let earth_surface = DiffuseLight::new(earth_texture);
        let globe = Sphere::new(Vec3::new(0., 0., 0.), 2., earth_surface);
        world.objects.push(globe);
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn simple_light(rng: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);
        let pertext = Arc::new(NoiseTexture::new(rng, 4.0));
        world.objects.push(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Lamberian::new(pertext.clone()),
        ));
        world.objects.push(Sphere::new(
            Vec3::new(0., 2., 0.),
            2.,
            Lamberian::new(pertext),
        ));

        let difflight = DiffuseLight::new(SolidColor::new(Vec3::new(4., 4., 4.)));
        world
            .objects
            .push(xy_rect::new(3., 5., 1., 3., -2., difflight));

        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(26., 3., 6.),
            Vec3::new(0., 2., 0.),
            Vec3::new(0., 1., 0.),
            20.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn cornell_box() -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);

        let red = Lamberian::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lamberian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lamberian::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(SolidColor::new(Vec3::new(15., 15., 15.)));

        world
            .objects
            .push(yz_rect::new(0., 555., 0., 555., 555., green));
        world
            .objects
            .push(yz_rect::new(0., 555., 0., 555., 0., red));
        world
            .objects
            .push(xz_rect::new(213., 343., 227., 332., 554., light));
        world
            .objects
            .push(xz_rect::new(0., 555., 0., 555., 0., white.clone()));
        world
            .objects
            .push(xz_rect::new(0., 555., 0., 555., 555., white.clone()));
        world
            .objects
            .push(xy_rect::new(0., 555., 0., 555., 555., white.clone()));

        let mut box1: Arc<dyn Hittable> = BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 330., 165.),
            white.clone(),
        );

        box1 = YRotation::new(box1, 15.);
        box1 = Translate::new(box1, Vec3::new(265., 0., 295.));
        box1 = ConstantMedium::new(
            box1,
            0.01,
            Lamberian::new(SolidColor::new(Vec3::new(0., 0., 0.))),
        );
        world.objects.push(box1);

        let mut box2: Arc<dyn Hittable> =
            BoxRender::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white);
        box2 = YRotation::new(box2, -18.);
        box2 = Translate::new(box2, Vec3::new(130., 0., 65.));
        box2 = ConstantMedium::new(
            box2,
            0.01,
            Lamberian::new(SolidColor::new(Vec3::new(1., 1., 1.))),
        );
        world.objects.push(box2);
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(278., 278., -800.),
            Vec3::new(278., 278., 0.),
            Vec3::new(0., 1., 0.),
            40.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn cornell_smoke() -> (Camera, Arc<HittableList>) {
        let mut world = HittableList::new(vec![]);

        let red = Lamberian::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lamberian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lamberian::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(SolidColor::new(Vec3::new(7., 7., 7.)));

        world
            .objects
            .push(yz_rect::new(0., 555., 0., 555., 555., green));
        world
            .objects
            .push(yz_rect::new(0., 555., 0., 555., 0., red));
        world
            .objects
            .push(xz_rect::new(113., 443., 127., 432., 554., light));
        world
            .objects
            .push(xz_rect::new(0., 555., 0., 555., 0., white.clone()));
        world
            .objects
            .push(xz_rect::new(0., 555., 0., 555., 555., white.clone()));
        world
            .objects
            .push(xy_rect::new(0., 555., 0., 555., 555., white.clone()));

        let mut box1: Arc<dyn Hittable> = BoxRender::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(165., 330., 165.),
            white.clone(),
        );
        box1 = YRotation::new(box1, 15.);
        box1 = Translate::new(box1, Vec3::new(265., 0., 295.));
        box1 = ConstantMedium::new(
            box1,
            0.01,
            Isotropic::new(SolidColor::new(Vec3::new(0., 0., 0.))),
        );
        world.objects.push(box1);

        let mut box2: Arc<dyn Hittable> =
            BoxRender::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white);
        box2 = YRotation::new(box2, -18.);
        box2 = Translate::new(box2, Vec3::new(130., 0., 65.));
        box2 = ConstantMedium::new(
            box2,
            0.01,
            Isotropic::new(SolidColor::new(Vec3::new(1., 1., 1.))),
        );
        world.objects.push(box2);
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(278., 278., -800.),
            Vec3::new(278., 278., 0.),
            Vec3::new(0., 1., 0.),
            40.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }

    pub fn final_scene(random: &mut Random<f64>) -> (Camera, Arc<HittableList>) {
        let mut boxes1 = HittableList::new(vec![]);
        let ground = Lamberian::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)));

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

                boxes1.objects.push(BoxRender::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    ground.clone(),
                ));
            }
        }

        let mut world = HittableList::new(vec![]);
        world
            .objects
            .push(Arc::new(BvhNode::new(&mut boxes1.objects, 0., 1.)));

        let light = DiffuseLight::new(SolidColor::new(Vec3::new(7., 7., 7.)));
        world
            .objects
            .push(xz_rect::new(123., 423., 147., 412., 554., light));

        let center1 = Vec3::new(400., 400., 200.);
        let center2 = center1 + Vec3::new(30., 0., 0.);
        let moving_sphere_material = Lamberian::new(SolidColor::new(Vec3::new(0.7, 0.3, 0.1)));
        world.objects.push(Arc::new(MovingSphere::new(
            center1,
            center2,
            0.,
            1.,
            50.,
            moving_sphere_material,
        )));

        world.objects.push(Sphere::new(
            Vec3::new(260., 150., 45.),
            50.,
            Dielectric::new(1.5),
        ));
        world.objects.push(Sphere::new(
            Vec3::new(0., 150., 145.),
            50.,
            Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.),
        ));

        let boundary = Sphere::new(Vec3::new(360., 150., 145.), 70., Dielectric::new(1.5));
        world.objects.push(boundary.clone());
        world.objects.push(ConstantMedium::new(
            boundary,
            0.2,
            Isotropic::new(SolidColor::new(Vec3::new(0.2, 0.4, 0.9))),
        ));
        let boundary2 = Sphere::new(Vec3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
        world.objects.push(ConstantMedium::new(
            boundary2,
            0.0001,
            Isotropic::new(SolidColor::new(Vec3::new(1., 1., 1.))),
        ));

        let emat = Lamberian::new(Arc::new(ImageTexture::new(Path::new("earthmap.jpg"))));
        world
            .objects
            .push(Sphere::new(Vec3::new(400., 200., 400.), 100., emat));
        let pertext = Arc::new(NoiseTexture::new(random, 0.1));
        world.objects.push(Sphere::new(
            Vec3::new(220., 280., 300.),
            80.,
            Lamberian::new(pertext),
        ));

        let mut boxes2 = HittableList::new(vec![]);
        let white = Lamberian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        const NS: usize = 1000;
        for _j in 0..NS {
            boxes2.objects.push(Sphere::new(
                Vec3::new(
                    165. * random.random(Some(0.), Some(1.)),
                    165. * random.random(Some(0.), Some(1.)),
                    165. * random.random(Some(0.), Some(1.)),
                ),
                10.,
                white.clone(),
            ));
        }

        world.objects.push(Translate::new(
            YRotation::new(Arc::new(BvhNode::new(&mut boxes2.objects, 0., 1.)), 15.),
            Vec3::new(-100., 270., 395.),
        ));
        let mut return_world = HittableList::new(vec![]);
        return_world
            .objects
            .push(Arc::new(BvhNode::new(&mut world.objects, 0., 1.)));
        let camera: Camera = Camera::new(
            Vec3::new(478., 278., -600.),
            Vec3::new(278., 278., 0.),
            Vec3::new(0., 1., 0.),
            40.0,
            ASPECT_RATIO,
            0.1,
            0.,
            1.,
        );
        (camera, Arc::from(return_world))
    }
}
