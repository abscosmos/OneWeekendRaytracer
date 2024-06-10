use glm::Vec3;
use rand::Rng;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;

pub struct Dielectric { pub refraction_index: f32 }

impl Dielectric {
    pub fn reflectance(cos: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let ri = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let reflect = Dielectric::reflectance(cos_theta, ri) > rand::rng().random_range(0.0..1.0);

        let direction = if ri * sin_theta > 1.0 || reflect {
            glm::reflect_vec(&unit_direction, &hit_record.normal)
        } else {
            glm::refract_vec(&unit_direction, &hit_record.normal, ri)
        };

        Some(
            ScatterResult {
                attenuation: Vec3::new(1.0, 1.0, 1.0),
                scattered: Ray { origin: hit_record.p, direction },
            }
        )
    }
}