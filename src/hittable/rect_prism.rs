use std::sync::Arc;
use glm::Vec3;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::quad::{make_box_from_opposite_vertices, Quad};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

pub struct RectangularPrism {
    faces: HittableList,
}

impl RectangularPrism {
    pub fn from_quads(quads: [Quad; 6]) -> Self {
        let mut faces = HittableList::default();
        quads.into_iter().for_each(|face| faces.add(Arc::new(face)));
        Self { faces }
    }

    pub fn from_opposite_vertices(a: Vec3, b: Vec3, material: Arc<dyn Material>) -> Self {
        Self::from_quads(make_box_from_opposite_vertices(a, b, material))
    }
}


impl Hittable for RectangularPrism {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        self.bounding_box().hit(ray, ray_t).then(|| self.faces.hit(ray, ray_t)).flatten()
    }

    fn bounding_box(&self) -> &AABB {
        self.faces.bounding_box()
    }
}
