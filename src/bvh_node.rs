use rayon::prelude::ParallelSliceMut;
use std::cmp::Ordering;

use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, objects::Hittable, ray::Ray};
#[derive(Default)]
pub struct BvhNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub bbox: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bbox.hit(ray, t_min, t_max)?;
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = if let Some(rec) = hit_left.as_ref() {
            self.right.hit(ray, t_min, rec.t)
        } else {
            self.right.hit(ray, t_min, t_max)
        };

        if let Some(rec) = hit_left {
            if let Some(rec_right) = hit_right {
                if rec.t < rec_right.t {
                    Some(rec)
                } else {
                    Some(rec_right)
                }
            } else {
                Some(rec)
            }
        } else {
            hit_right
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}

impl BvhNode {
    pub fn new(src_objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let mut output = Self::default();
        let length = src_objects.len();
        if length == 1 {
            output.left = Some(src_objects[0].clone());
            output.right = Some(src_objects[0].clone());
        } else if length == 2 {
            if Self::box_compare(&src_objects[0], &src_objects[1]) == Ordering::Less {
                output.left = Some(src_objects[0].clone());
                output.right = Some(src_objects[1].clone());
            } else {
                output.left = Some(src_objects[1].clone());
                output.right = Some(src_objects[0].clone());
            }
        } else {
            src_objects.par_sort_unstable_by(Self::box_compare);
            let mid = length / 2;
            let (new_left, new_right) = rayon::join(
                || BvhNode::new(&mut src_objects[0..mid].to_vec(), time0, time1),
                || BvhNode::new(&mut src_objects[mid..length].to_vec(), time0, time1),
            );
            output.left = Some(Arc::new(new_left));
            output.right = Some(Arc::new(new_right));
        }
        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();
        Self::get_surrounding_box(&output, time0, time1, &mut box_left, &mut box_right);

        output.bbox = Aabb::surrounding_box(box_left, box_right);
        output
    }
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();
        let (a_result, b_result) = rayon::join(
            || a.bounding_box(0.0, 0.0, &mut box_a),
            || b.bounding_box(0.0, 0.0, &mut box_b),
        );
        if !a_result || !b_result {
            panic!("No bounding box in BvhNode constructor.");
        }

        box_a.partial_cmp(&box_b).unwrap()
    }
    fn get_surrounding_box(
        output: &BvhNode,
        time0: f64,
        time1: f64,
        box_left: &mut Aabb,
        box_right: &mut Aabb,
    ) {
        let (a_result, b_result) = rayon::join(
            || output.left.bounding_box(time0, time1, box_left),
            || output.right.bounding_box(time0, time1, box_right),
        );
        if !a_result || !b_result {
            panic!("No bounding box in BvhNode constructor.");
        }
    }
}
