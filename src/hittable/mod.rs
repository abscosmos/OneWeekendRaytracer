use std::sync::Arc;
use glm::{Vec3, Vec2};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::aabb::AABB;

pub mod hittable_list;
pub mod sphere;
pub mod bvh;
pub mod quad;
pub mod transform;
pub mod constant_medium;
pub mod rect_prism;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

#[derive(Clone)]
pub struct HitRecord {
    pub material: Arc<dyn Material>,
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub uv: Vec2,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_with_outward_normal(p: Vec3, t: f32, uv: Vec2, ray: Ray, outward_normal: Vec3, material: Arc<dyn Material>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        HitRecord {
            material,
            p,
            front_face,
            t, uv,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}
