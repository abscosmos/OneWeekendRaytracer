use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let mut aabb = AABB::default();

        for object in objects.iter() {
            aabb = aabb.enclosing(object.bounding_box());
        }

        let axis = aabb.longest_axis();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objects.len() {
            1 => {
                ( objects[0].clone(), objects[0].clone() )
            }
            2 => {
                ( objects[0].clone(), objects[1].clone() )
            }
            n => {
                objects.sort_by(|a, b|
                    a.bounding_box().compare_by_axis(b.bounding_box(), axis)
                );

                let mid = n / 2;

                (
                    Arc::new(BVHNode::new(&mut objects[..mid])),
                    Arc::new(BVHNode::new(&mut objects[mid..])),
                )
            }
        };

        let aabb = AABB::enclosing(left.bounding_box(), right.bounding_box());

        Self {
            left,
            right,
            aabb,
        }
    }

    pub fn box_compare_along_axis(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> bool {
        a.bounding_box().axis_interval(axis).min < b.bounding_box().axis_interval(axis).min
    }
}

impl From<HittableList> for BVHNode {
    fn from(mut value: HittableList) -> Self {
        assert!(!value.objects.is_empty());
        Self::new(&mut value.objects)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.aabb.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let hit_right = self.right.hit(
            ray, Interval {
                min: ray_t.min,
                max: hit_left.as_ref().map_or(ray_t.max, |rec| rec.t)
            }
        );

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}