use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::util::vec3_random::random_vec_in_unit_sphere;

pub struct Isotropic {
    pub texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_albedo(color: Color) -> Self {
        Self { texture: Arc::new(SolidColor { albedo: color }) }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let scattered = Ray {
            origin: hit_record.p,
            direction: random_vec_in_unit_sphere().normalize(),
        };

        let attenuation = self.texture.value(hit_record.uv, hit_record.p);

        Some(ScatterResult { attenuation, scattered })
    }
}