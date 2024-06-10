pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;

use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

// TODO: make enum with dynamic material

pub trait Material: Send + Sync {
    #[expect(unused_variables, reason = "trait method")]
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        None
    }

    #[expect(unused_variables, reason = "trait method")]
    fn emitted(&self, uv: Vec2, p: Vec3) -> Color {
        Color::default()
    }
}
