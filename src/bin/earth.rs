use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

fn main() {
    let mut world = HittableList::default();

    world.add(Arc::new(Sphere::new(
        Vec3::zeros(),
        2.0,
        Arc::new(Lambertian {
            texture: Arc::new(ImageTexture {
                image: image::open("./assets/earthmap.jpg")
                    .expect("earth map asset should exist")
                    .to_rgb8(),
            }),
        })
    )));

    let cam = Camera::new(
        Vec3::new(8.0, 0.0, 12.0),
        Vec3::zeros(),
        Color::new(0.70, 0.80, 1.00),
        CameraDimensions::FHD,
        RenderQuality::HIGH,
        FocusSettings::default(),
        20.0,
    );

    cam.render_screen_par(&world, 20, Some("earth"))
        .save("earth.png")
        .expect("failed to save file")
}