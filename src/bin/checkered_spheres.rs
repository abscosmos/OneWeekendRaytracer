use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main()  {
    let mut world = HittableList::default();

    let checker_texture = Arc::new(Lambertian {
        texture: Arc::new(CheckerTexture::from_colors(
            0.32,
            Color::new(0.2, 0.3, 0.1),
            Color::from_element(0.9),
        ))
    });

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        checker_texture.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        checker_texture.clone(),
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


    let world: BVHNode = world.into();

    cam.render_screen_par(&world, 20, Some("checkered_spheres"))
        .save("checkered_spheres.png")
        .expect("failed to save file")
}