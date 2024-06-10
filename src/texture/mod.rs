pub mod solid_color;
pub mod checker;
pub mod image;
pub mod perlin;
pub mod blend;

use glm::{Vec2, Vec3};
use crate::color::Color;

pub trait Texture: Send + Sync {
    fn value(&self, uv: Vec2, p: Vec3) -> Color;
}