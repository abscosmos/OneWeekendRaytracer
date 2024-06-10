use glm::Vec3;
use rand::distr::uniform::SampleRange;
use rand::Rng;

pub fn random_vec(range: impl SampleRange<f32> + Clone) -> Vec3 {
    let mut rng = rand::rng();

    Vec3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        rng.random_range(range),
    )
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let v = random_vec(-1.0..=1.0f32);

        if v.magnitude_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_vec_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_vec_in_unit_sphere().normalize();

    if on_unit_sphere.dot(&normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_vec_in_unit_disk() -> Vec3 {
    let mut rng = rand::rng();

    loop {
        let v = Vec3::new(
            rng.random_range(-1.0..=1.0f32),
            rng.random_range(-1.0..=1.0f32),
            0.0,
        );

        if v.magnitude_squared() < 1.0 {
            return v;
        }
    }
}