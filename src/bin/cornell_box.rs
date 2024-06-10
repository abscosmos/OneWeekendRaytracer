use std::num::NonZero;
use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main() {
    let mut world = HittableList::default();

    let red_mat = Arc::new(Lambertian::from_albedo(
        Color::new(0.65, 0.05, 0.05)
    ));

    let green_mat = Arc::new(Lambertian::from_albedo(
        Color::new(0.12, 0.45, 0.15)
    ));

    let white_mat = Arc::new(Lambertian::from_albedo(
        Color::from_element(0.73)
    ));

    let light_mat = Arc::new(DiffuseLight::from_emission_color(
        Color::from_element(15.0)
    ));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green_mat
    )));

    world.add(Arc::new(Quad::new(
        Vec3::default(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red_mat
    )));

    world.add(Arc::new(Quad::new(
        Vec3::default(),
        Vec3::new(555.0,0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white_mat.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vec3::from_element(555.0),
        Vec3::new(-555.0,0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white_mat.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white_mat.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light_mat
    )));

    world.add(Arc::new(Transform::new(
        Arc::new(RectangularPrism::from_opposite_vertices(
            Vec3::default(),
            Vec3::new(165.0, 330.0, 165.0),
            white_mat.clone(),
        )),
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(0.0, -15.0, 0.0)
    )));

    world.add(Arc::new(Transform::new(
        Arc::new(RectangularPrism::from_opposite_vertices(
            Vec3::default(),
            Vec3::from_element(165.0),
            white_mat.clone(),
        )),
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(0.0, 18.0, 0.0),
    )));

    let cam = Camera::new(
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Color::default(),
        CameraDimensions::square(NonZero::new(800).expect("nonzero")),
        RenderQuality::HIGH,
        FocusSettings::default(),
        40.0
    );

    cam.render_screen_par(&world, 20, Some("cornell_box"))
        .save("cornell_box.png")
        .expect("failed to save file")
}