use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::texture::Texture;

pub struct SolidColor {
    pub albedo: Color,
}

impl Texture for SolidColor {
    fn value(&self, _uv: Vec2, _p: Vec3) -> Color {
        self.albedo
    }
}