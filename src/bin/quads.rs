use std::num::NonZero;
use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main() {
    let mut world = HittableList::default();

    // left
    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::from_albedo(
            Color::new(1.0, 0.2, 0.2),
        ))
    )));

    // back
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::from_albedo(
            Color::new(0.2, 1.0, 0.2),
        ))
    )));

    // right
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::from_albedo(
            Color::new(0.2, 0.2, 1.0),
        ))
    )));

    // upper
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(Lambertian::from_albedo(
            Color::new(1.0, 0.5, 0.0),
        ))
    )));

    // lower

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(Lambertian::from_albedo(
            Color::new(0.2, 0.8, 0.8),
        ))
    )));

    let cam = Camera::new(
        Vec3::new(0.0, 0.0, 9.0),
        Vec3::default(),
        Color::new(0.70, 0.80, 1.00),
        CameraDimensions::square(NonZero::new(800).expect("nonzero")),
        RenderQuality::HIGH,
        FocusSettings::default(),
        80.0,
    );

    cam.render_screen_par(&world, 20, Some("quads"))
        .save("quads.png")
        .expect("failed to save file")
}