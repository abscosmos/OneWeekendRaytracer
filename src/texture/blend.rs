use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::texture::Texture;

pub struct BlendedTexture {
    pub a: Arc<dyn Texture>,
    pub b: Arc<dyn Texture>,
    pub blend: Arc<dyn Fn(Color, Color) -> Color + Send + Sync>
}

impl Texture for BlendedTexture {
    fn value(&self, uv: Vec2, p: Vec3) -> Color {
        (self.blend) (
            self.a.value(uv, p),
            self.b.value(uv, p),
        )
    }
}