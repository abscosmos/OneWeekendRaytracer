use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;

pub struct CheckerTexture {
    pub inv_scale: f32,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_textures(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        assert!(scale > 0.0, "scale must be positive");
        
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f32, c1: Color, c2: Color) -> Self {
        Self::from_textures(
            scale,
            Arc::new(SolidColor { albedo: c1 }),
            Arc::new(SolidColor { albedo: c2 }),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: Vec2, p: Vec3) -> Color {
        let scaled = p * self.inv_scale;

        let sum = scaled.x.floor() as i32 + scaled.y.floor() as i32 + scaled.z.floor() as i32;

        match sum % 2 == 0 {
            true => self.even.value(uv, p),
            false => self.odd.value(uv, p),
        }
    }
}