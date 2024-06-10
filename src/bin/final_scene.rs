use std::num::NonZero;
use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

pub fn main() {
    let mut world = HittableList::default();

    let ground_material = Arc::new(
        Lambertian::from_albedo(Color::new(0.48, 0.83, 0.53))
    );
    
    let boxes_per_side = 20;

    let mut boxes = HittableList::default();

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;

            let x1 = x0 + w;
            let y1 = rand::random_range(1.0..101.0);
            let z1 = z0 + w;
            let prism = RectangularPrism::from_opposite_vertices(
                Vec3::new(x0, 0.0, z0),
                Vec3::new(x1, y1, z1),
                ground_material.clone()
            );
            boxes.add(Arc::new(prism));
        }
    }

    let boxes: BVHNode = boxes.into();

    world.add(Arc::new(boxes));

    world.add(Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Arc::new(DiffuseLight::from_emission_color(
            Color::from_element(7.0)
        ))
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 400.0, 200.0),
        50.0,
        Arc::new(Lambertian::from_albedo(
            Color::new(0.7, 0.3, 0.1)
        ))
    )));

    let dielectric = Arc::new(Dielectric { refraction_index: 1.5 });

    world.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal {
            albedo: Color::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        })
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        dielectric.clone(),
    ));

    world.add(boundary.clone());

    world.add(Arc::new(ConstantMedium::from_albedo(
        boundary, 0.2, Color::new(0.2, 0.4, 0.9)
    )));

    world.add(Arc::new(ConstantMedium::from_albedo(
        Arc::new(Sphere::new(
            Vec3::zeros(),
            5000.0,
            dielectric.clone(),
        )),
        0.0001,
        Color::from_element(1.0),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Lambertian {
            texture: Arc::new(ImageTexture {
                image: image::open("./assets/earthmap.jpg")
                    .expect("earth map asset should exist")
                    .to_rgb8(),
            }),
        })
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian {
            texture: Arc::new(NoiseTexture::new(0.2, 7)),
        }),
    )));

    let mut small_spheres = HittableList::default();

    let white_material = Arc::new(Lambertian::from_albedo(Color::from_element(0.73)));

    for _ in 0..1000 {
        small_spheres.add(Arc::new(Sphere::new(
            Vec3::from_fn(|_, _| rand::random_range(0.0..165.0) ),
            10.0,
            white_material.clone()
        )));
    }

    world.add(Arc::new(Transform::new(
        Arc::<BVHNode>::new(small_spheres.into()),
        Vec3::new(-100.0, 270.0, 395.0),
        Vec3::new(0.0, 15.0, 0.0),
    )));

    let cam = Camera::new(
        Vec3::new(478.0, 278.0, -600.0),
        Vec3::new(278.0, 278.0, 0.0),
        Color::default(),
        CameraDimensions::square(NonZero::new(800).expect("nonzero")),
        RenderQuality { samples_per_pixel: 5000, max_depth: 40 },
        FocusSettings::default(),
        40.0
    );

    cam.render_screen_par(&world, 5, Some("final_scene"))
        .save("final_scene.png")
        .expect("failed to save file")
}