use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::material::Material;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;

pub struct DiffuseLight {
    pub texture: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn from_emission_color(emissive_color: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor {
                albedo: emissive_color,
            })
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: Vec2, p: Vec3) -> Color {
        self.texture.value(uv, p)
    }
}