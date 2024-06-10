use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    aabb: AABB,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.aabb = self.aabb.enclosing(object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut curr_closest = ray_t.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, Interval { min: ray_t.min, max: curr_closest }) {
                curr_closest = rec.t;
                hit_rec = Some(rec);
            }
        }

        hit_rec
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}