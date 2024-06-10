pub mod hittable;
pub mod material;
pub mod texture;
pub mod aabb;
pub mod camera;
pub mod color;
pub mod interval;
pub mod ray;
pub mod util;

extern crate nalgebra_glm as glm;
extern crate nalgebra as na;

#[allow(unused_imports, reason = "prelude")]
pub mod prelude {
    pub use super::camera::{
        Camera,
        dimensions::CameraDimensions,
        focus_settings::FocusSettings,
        render_quality::RenderQuality
    };
    pub use super::color::Color;
    pub use super::hittable::{
        bvh::BVHNode,
        constant_medium::ConstantMedium,
        hittable_list::HittableList,
        quad::Quad,
        rect_prism::RectangularPrism,
        sphere::Sphere,
        transform::Transform,
    };
    pub use super::material::{
        Material,
        dielectric::Dielectric,
        diffuse_light::DiffuseLight,
        lambertian::Lambertian,
        metal::Metal,
    };
    pub use super::texture::{
        blend::BlendedTexture,
        checker::CheckerTexture,
        image::ImageTexture,
        perlin::{MarbleTexture, NoiseTexture},
        solid_color::SolidColor,
    };
    pub use super::util::vec3_random as rand_vec;
    pub use glm::Vec3;
}