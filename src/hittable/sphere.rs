use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material>,
    aabb: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        assert!(radius > 0.0, "Sphere radius must be positive");
        
        let r_vec = Vec3::from_element(radius);

        Self {
            center,
            radius,
            material,
            aabb: AABB::from_extrema(
                center - r_vec,
                center + r_vec,
            ),
        }
    }

    pub fn get_uv(p: Vec3) -> Vec2 {
        use std::f32::consts::{ PI, TAU };

        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + PI;

        Vec2::new( phi / TAU, theta / PI )
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.magnitude_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (h - sqrt_discriminant) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);

        let outward_normal = (p - self.center) / self.radius;
        
        Some(
            HitRecord::new_with_outward_normal(
                p,
                root,
                Self::get_uv(outward_normal),
                ray,
                outward_normal,
                self.material.clone()
            )
        )
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}