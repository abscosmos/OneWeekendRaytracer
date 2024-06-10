use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main() {
    let mut world = HittableList::default();

    let perlin_texture = Arc::new(Lambertian {
        texture: Arc::new(NoiseTexture::new(4.0, 7)),
    });

    let red = Arc::new(Lambertian {
        texture: Arc::new(BlendedTexture {
            a: Arc::new(SolidColor { albedo: Color::new(0.8, 0.1, 0.04) }),
            b: Arc::new(NoiseTexture::new(4.0, 4)),
            blend: Arc::new(|a, b|
                b.x.sqrt() * a
            )
        }),
    });

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0), 1000.0, perlin_texture.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0), 2.0, red.clone(),
    )));

    let diffuse_light = Arc::new(DiffuseLight::from_emission_color(
        Color::from_element(4.0),
    ));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));

    let cam = Camera::new(
        Vec3::new(26.0, 3.0, 6.0),
        Vec3::new(0.0, 2.0, 0.0),
        Color::default(),
        CameraDimensions::FHD,
        RenderQuality::HIGH,
        FocusSettings::default(),
        20.0,
    );

    cam.render_screen_par(&world, 20, Some("simple_light"))
        .save("simple_light.png")
        .expect("failed to save file")
}