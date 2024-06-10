use std::sync::Arc;
use glm::Vec3;
use na::Rotation3;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Transform {
    object: Arc<dyn Hittable>,
    translation: Vec3,
    rotation: Rotation3<f32>,
    aabb: AABB,
}

impl Transform {
    pub fn new(object: Arc<dyn Hittable>, translation: Vec3, angles: Vec3) -> Self {
        let aabb = object.bounding_box();

        let mut max = Vec3::from_element(f32::INFINITY);
        let mut min = Vec3::from_element(f32::NEG_INFINITY);

        let rotation = Rotation3::new(angles.map(f32::to_radians));

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let loc = Vec3::new(i as f32, j as f32, k as f32);

                    let vec = rotation * Vec3::from_fn(|i, _|
                        loc[i] * aabb[i].max + (1.0 - loc[i]) * aabb[i].min
                    );

                    for n in 0..3 {
                        min[n] = min[n].min(vec[n]);
                        max[n] = max[n].max(vec[n]);
                    }
                }
            }
        }

        Self {
            object,
            translation,
            rotation,
            aabb: AABB::from_extrema(min, max) + translation,
        }
    }


}

impl Hittable for Transform {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = self.rotation * Ray {
            origin: ray.origin - self.translation,
            .. ray
        };

        if let Some(rec) = self.object.hit(offset_ray, ray_t) {
            let rot_transpose = self.rotation.transpose();

            Some(
                HitRecord {
                    p: rot_transpose * rec.p + self.translation,
                    normal: rot_transpose * rec.normal,
                    .. rec
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}