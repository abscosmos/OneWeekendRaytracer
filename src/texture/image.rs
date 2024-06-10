use glm::{Vec2, Vec3};
use image::RgbImage;
use crate::color::Color;
use crate::texture::Texture;

pub struct ImageTexture {
    pub image: RgbImage,
}

impl Texture for ImageTexture {
    fn value(&self, uv: Vec2, _p: Vec3) -> Color {
        let u = uv.x.clamp(0.0,1.0);
        let v = 1.0 - (uv.y.clamp(0.0,1.0));

        let pixel = self.image.get_pixel(
            ((u * self.image.width() as f32) as u32).clamp(0, self.image.width() -1),
            ((v * self.image.height() as f32) as u32).clamp(0, self.image.height() -1),
        );

        Color::from_row_slice(&pixel.0.map(|c| c as f32)) * 1.0 / 255.0
    }
}