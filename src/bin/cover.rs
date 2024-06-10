use std::ops::Range;
use std::sync::Arc;
use one_weekend_raytracer::prelude::*;

fn main() {
    let mut world = HittableList::default();

    const UNIT_RANGE: Range<f32> = 0.0..1.0f32;

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            // texture: Arc::new(CheckerTexture::from_colors(
            //     1.0,
            //     Color::new(0.2, 0.3, 0.1),
            //     Color::from_element(0.9),
            // )),

            texture: Arc::new(BlendedTexture {
                a: Arc::new(SolidColor { albedo: Color::new(0.25, 0.61, 0.04) }),
                b: Arc::new(NoiseTexture::new(4.0, 4)),
                blend: Arc::new(|a, b|
                    b.x.sqrt() * a
                ),
            })
        }),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rand::random_range(UNIT_RANGE),
                0.2,
                b as f32 + 0.9 * rand::random_range(UNIT_RANGE),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Arc<dyn Material> = match rand::random_range(UNIT_RANGE) {
                    0.0..=0.8 => {
                        Arc::new(Lambertian::from_albedo(
                            Color::from_fn(|_,_| rand::random_range(UNIT_RANGE) * rand::random_range(UNIT_RANGE))
                        ))
                    }
                    0.8..=0.95 => {
                        Arc::new(Metal {
                            albedo: rand_vec::random_vec(0.5..1.0),
                            fuzz: rand::random_range(0.0..0.5),
                        })
                    }
                    0.95..=1.0 => {
                        Arc::new(Dielectric {
                            refraction_index: 1.5,
                        })
                    }
                    _ => unreachable!(),
                };

                world.add(Arc::new(Sphere::new( center, 0.2, material )));
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_albedo(Color::new(0.4, 0.2, 0.1)),
        ))));

    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    )));

    let cam = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::zeros(),
        Color::new(0.70, 0.80, 1.00),
        CameraDimensions::FHD,
        RenderQuality::HIGH,
        FocusSettings { defocus_angle: 0.6, focus_dist: 10.0 },
        20.0,
    );

    let world: BVHNode = world.into();

    cam.render_screen_par(&world, 20, Some("cover"))
        .save("cover.png")
        .expect("failed to save file")
}