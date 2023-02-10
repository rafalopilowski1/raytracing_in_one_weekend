use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, HittableThreadSafe},
    objects::Hittable,
    ray::Ray,
};
#[derive(Default)]
pub struct BvhNode {
    pub left: Option<HittableThreadSafe>,
    pub right: Option<HittableThreadSafe>,
    pub bbox: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max, rec) {
            return false;
        }
        let mut hit_left = false;
        if let Some(left) = &self.left {
            hit_left = left.hit(ray, t_min, t_max, rec);
        }
        let mut hit_right = false;
        if let Some(right) = &self.right {
            hit_right = right.hit(ray, t_min, t_max, rec);
        }
        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}

impl BvhNode {
    fn new(
        src_objects: &mut [HittableThreadSafe],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut output: BvhNode = Default::default();
        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let object_span = end - start;
        if object_span == 1 {
            output.left = Some(src_objects[start].clone());
            output.right = Some(src_objects[start].clone());
        } else if object_span == 2 {
            if comparator(&src_objects[start], &src_objects[start + 1]) == std::cmp::Ordering::Less
            {
                output.left = Some(src_objects[start].clone());
                output.right = Some(src_objects[start + 1].clone());
            } else {
                output.left = Some(src_objects[start + 1].clone());
                output.right = Some(src_objects[start].clone());
            }
        } else {
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            output.left = Some(Arc::new(BvhNode::new(
                src_objects,
                start,
                mid,
                time0,
                time1,
            )));
            output.right = Some(Arc::new(BvhNode::new(src_objects, mid, end, time0, time1)));
        }
        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();
        if let (Some(left), Some(right)) = (&output.left, &output.right) {
            if !left.bounding_box(time0, time1, &mut box_left)
                || !right.bounding_box(time0, time1, &mut box_right)
            {
                panic!("No bounding box in BvhNode constructor.");
            }
        }

        output.bbox = Aabb::surrounding_box(box_left, box_right);
        output
    }
    fn box_compare(a: &HittableThreadSafe, b: &HittableThreadSafe, axis: usize) -> bool {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BvhNode constructor.");
        }
        box_a.min[axis] < box_b.min[axis]
    }
    fn box_x_compare(a: &HittableThreadSafe, b: &HittableThreadSafe) -> std::cmp::Ordering {
        if Self::box_compare(a, b, 0) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
    fn box_y_compare(a: &HittableThreadSafe, b: &HittableThreadSafe) -> std::cmp::Ordering {
        if Self::box_compare(a, b, 1) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
    fn box_z_compare(a: &HittableThreadSafe, b: &HittableThreadSafe) -> std::cmp::Ordering {
        if Self::box_compare(a, b, 2) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
