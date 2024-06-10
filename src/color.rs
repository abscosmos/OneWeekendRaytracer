use glm::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

pub fn transform_color_to_pixel(color: Color) -> image::Rgb<u8> {
    const INTENSITY: Interval = Interval { min: 0.0, max: 0.999 };

    let transformed_color = color.map(|v|
        INTENSITY.clamp(v.max(0.0).sqrt()) * 256.0
    );

    image::Rgb([
        transformed_color.x as u8,
        transformed_color.y as u8,
        transformed_color.z as u8,
    ])
}