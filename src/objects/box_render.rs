use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, HittableList},
    material::Material,
    vec3::Vec3,
};

use crate::{
    objects::rect::{xy_rect, xz_rect, yz_rect},
    Hittable,
};

pub struct BoxRender {
    pmin: Vec3,
    pmax: Vec3,
    sides: HittableList,
}

impl BoxRender {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Arc<Self> {
        let mut sides = HittableList::new(vec![]);
        sides.objects.push(xy_rect::new(
            p0.x_r,
            p1.x_r,
            p0.y_g,
            p1.y_g,
            p1.z_b,
            material.clone(),
        ));
        sides.objects.push(xy_rect::new(
            p0.x_r,
            p1.x_r,
            p0.y_g,
            p1.y_g,
            p0.z_b,
            material.clone(),
        ));
        sides.objects.push(xz_rect::new(
            p0.x_r,
            p1.x_r,
            p0.z_b,
            p1.z_b,
            p1.y_g,
            material.clone(),
        ));
        sides.objects.push(xz_rect::new(
            p0.x_r,
            p1.x_r,
            p0.z_b,
            p1.z_b,
            p0.y_g,
            material.clone(),
        ));
        sides.objects.push(yz_rect::new(
            p0.y_g,
            p1.y_g,
            p0.z_b,
            p1.z_b,
            p1.x_r,
            material.clone(),
        ));
        sides.objects.push(yz_rect::new(
            p0.y_g,
            p1.y_g,
            p0.z_b,
            p1.z_b,
            p0.x_r,
            material.clone(),
        ));
        Arc::from(Self {
            pmin: p0,
            pmax: p1,
            sides,
        })
    }
}

impl Hittable for BoxRender {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.pmin, self.pmax);
        true
    }
}
