use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main() {
    let mut world = HittableList::default();

    let perlin_tex = Arc::new(Lambertian {
        texture: Arc::new(MarbleTexture::new(4.0, 7)),
    });

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_tex.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_tex.clone(),
    )));

    let cam = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::zeros(),
        Color::new(0.70, 0.80, 1.00),
        CameraDimensions::FHD,
        RenderQuality::HIGH,
        FocusSettings::default(),
        20.0,
    );

    cam.render_screen_par(&world, 20, Some("perlin_spheres"))
        .save("perlin_spheres.png")
        .expect("failed to save file")
}