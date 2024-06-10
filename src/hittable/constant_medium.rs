use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval;
use crate::interval::Interval;
use crate::material::isotropic::Isotropic;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub neg_inv_density: f32,
    pub phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub const HIT_EPSILON: f32 = 0.0001;
    
    pub fn new_isotropic(boundary: Arc<dyn Hittable>, density: f32, phase_function: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic {
                texture: phase_function
            }),
        }
    }

    pub fn from_albedo(boundary: Arc<dyn Hittable>, density: f32, albedo: Color) -> Self {
        Self::new_isotropic(boundary, density, Arc::new(SolidColor { albedo }))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(ray, interval::UNIVERSE)?;

        let mut rec2 = self.boundary.hit(ray, Interval { min: rec1.t + Self::HIT_EPSILON, max: f32::INFINITY })?;

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction.magnitude();
        let dist_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random_range(0.0f32..1.0).ln();

        if hit_distance > dist_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;

        Some(
            HitRecord {
                material: self.phase_function.clone(),
                normal: Vec3::new(1.0, 0.0, 0.0),
                front_face: true,
                p: ray.at(t),
                t,
                uv: Vec2::default(),
            }
        )
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}