use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::util::vec3_random::random_vec_in_unit_sphere;

pub struct Lambertian { pub texture: Arc<dyn Texture> }

impl Lambertian {
    pub const SCATTER_EPSILON: f32 = 1e-8;
    
    pub fn from_albedo(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor { albedo }),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut direction = hit_record.normal + random_vec_in_unit_sphere().normalize();

        if direction.magnitude_squared() < Self::SCATTER_EPSILON {
            direction = hit_record.normal;
        }

        Some(
            ScatterResult {
                scattered: Ray { origin: hit_record.p, direction },
                attenuation: self.texture.value(hit_record.uv, hit_record.p),
            }
        )
    }
}