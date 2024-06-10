use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::util::vec3_random::random_vec_in_unit_sphere;

pub struct Metal { pub albedo: Color, pub fuzz: f32 }

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let direction = glm::reflect_vec(&ray.direction, &hit_record.normal)
            .normalize() + (self.fuzz * random_vec_in_unit_sphere().normalize());

        (direction.dot(&hit_record.normal) > 0.0)
            .then_some(ScatterResult {
                scattered: Ray { origin: hit_record.p, direction },
                attenuation: self.albedo,
            })
    }
}