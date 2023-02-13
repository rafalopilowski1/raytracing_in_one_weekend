use rayon::prelude::ParallelSliceMut;
use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, objects::Hittable, ray::Ray};
#[derive(Default)]
pub struct BvhNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
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

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}

impl BvhNode {
    pub fn new(src_objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let mut output = Self::default();
        let start = 0;
        let end = src_objects.len();
        let object_span = end - start;
        if object_span == 1 {
            output.left = Some(src_objects[start].clone());
            output.right = Some(src_objects[start].clone());
        } else if object_span == 2 {
            if Self::box_compare(&src_objects[start], &src_objects[start + 1])
                == std::cmp::Ordering::Less
            {
                output.left = Some(src_objects[start].clone());
                output.right = Some(src_objects[start + 1].clone());
            } else {
                output.left = Some(src_objects[start + 1].clone());
                output.right = Some(src_objects[start].clone());
            }
        } else {
            src_objects.par_sort_unstable_by(Self::box_compare);
            let mid = start + object_span / 2;
            output.left = Some(Arc::new(BvhNode::new(
                &mut src_objects[start..mid],
                time0,
                time1,
            )));
            output.right = Some(Arc::new(BvhNode::new(
                &mut src_objects[mid..end],
                time0,
                time1,
            )));
        }
        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();
        Self::get_surrounding_box(&output, time0, time1, &mut box_left, &mut box_right);

        output.bbox = Aabb::surrounding_box(box_left, box_right);
        output
    }
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BvhNode constructor.");
        }

        box_a.min[0]
            .total_cmp(&box_b.min[0])
            .then(box_a.min[1].total_cmp(&box_b.min[1]))
            .then(box_a.min[2].total_cmp(&box_b.min[2]))
    }
    fn get_surrounding_box(
        output: &BvhNode,
        time0: f64,
        time1: f64,
        box_left: &mut Aabb,
        box_right: &mut Aabb,
    ) {
        if let (Some(left), Some(right)) = (&output.left, &output.right) {
            if !left.bounding_box(time0, time1, box_left)
                || !right.bounding_box(time0, time1, box_right)
            {
                panic!("No bounding box in BvhNode constructor.");
            }
        }
    }
}
