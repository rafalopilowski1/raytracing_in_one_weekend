use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, HittableList},
    material::Material,
    vec3::Vec3,
};

use super::{xy_rect::xy_rect, xz_rect::xz_rect, yz_rect::yz_rect, Hittable};

pub struct BoxRender {
    pmin: Vec3,
    pmax: Vec3,
    sides: HittableList,
}

impl BoxRender {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new(vec![]);
        sides.objects.push(Arc::new(xy_rect::new(
            p0.x_r,
            p1.x_r,
            p0.y_g,
            p1.y_g,
            p1.z_b,
            material.clone(),
        )));
        sides.objects.push(Arc::new(xy_rect::new(
            p0.x_r,
            p1.x_r,
            p0.y_g,
            p1.y_g,
            p0.z_b,
            material.clone(),
        )));
        sides.objects.push(Arc::new(xz_rect::new(
            p0.x_r,
            p1.x_r,
            p0.z_b,
            p1.z_b,
            p1.y_g,
            material.clone(),
        )));
        sides.objects.push(Arc::new(xz_rect::new(
            p0.x_r,
            p1.x_r,
            p0.z_b,
            p1.z_b,
            p0.y_g,
            material.clone(),
        )));
        sides.objects.push(Arc::new(yz_rect::new(
            p0.y_g,
            p1.y_g,
            p0.z_b,
            p1.z_b,
            p1.x_r,
            material.clone(),
        )));
        sides.objects.push(Arc::new(yz_rect::new(
            p0.y_g,
            p1.y_g,
            p0.z_b,
            p1.z_b,
            p0.x_r,
            material.clone(),
        )));
        Self {
            pmin: p0,
            pmax: p1,
            sides,
        }
    }
}

impl Hittable for BoxRender {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.pmin, self.pmax);
        true
    }
}