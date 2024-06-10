use std::sync::Arc;
use glm::{Vec2, Vec3};
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable::hittable_list::HittableList;
use crate::interval;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

// TODO: flat primitive struct

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f32,
    material: Arc<dyn Material>,
    aabb: AABB,
}

impl Quad {
    pub const PARALLEL_THRESHOLD: f32 = 1e-8;

    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        assert!(
            u.magnitude_squared() >= f32::EPSILON && v.magnitude_squared() >= f32::EPSILON,
            "Quad edges must be non-zero vectors"
        );

        let diag1 = AABB::from_extrema(q, q + u + v);
        let diag2 = AABB::from_extrema(q + u, q + v);
        let aabb = diag1.enclosing(&diag2);

        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.magnitude_squared();

        Self {
            q, u, v, w,
            normal, d,
            material,
            aabb,
        }
    }

    fn is_interior(a: f32, b: f32) -> bool {
        interval::UNIT.contains(a) && interval::UNIT.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() < Self::PARALLEL_THRESHOLD {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denominator;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_point_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_point_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point_vector));
        
        let uv = Vec2::new(alpha, beta);
        
        if !Self::is_interior(alpha, beta) {
            return None;
        }

        Some(HitRecord::new_with_outward_normal(
            ray.at(t),
            t,
            uv,
            ray,
            self.normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

pub fn make_box_from_opposite_vertices(a: Vec3, b: Vec3, material: Arc<dyn Material>) -> [Quad; 6] {
    let min = Vec3::from_fn(|i, _| a[i].min(b[i]) );
    let max = Vec3::from_fn(|i, _| a[i].max(b[i]) );

    let diff = max - min;

    let x = Vec3::new(diff.x, 0.0, 0.0);
    let y = Vec3::new(0.0, diff.y, 0.0);
    let z = Vec3::new(0.0, 0.0, diff.z);

    [
        Quad::new(
            Vec3::new(min.x, min.y, max.z),
            x, y,
            material.clone(),
        ),
        Quad::new(
            Vec3::new(max.x, min.y, max.z),
            -z, y,
            material.clone(),
        ),
        Quad::new(
            Vec3::new(max.x, min.y, min.z),
            -x, y,
            material.clone(),
        ),
        Quad::new(
            min,
            z, y,
            material.clone(),
        ),
        Quad::new(
            Vec3::new(min.x, max.y, max.z),
            x, -z,
            material.clone(),
        ),
        Quad::new(
            min,
            x, z,
            material,
        ),
    ]
}

impl From<[Quad; 6]> for HittableList {
    fn from(value: [Quad; 6]) -> Self {
        let mut list = HittableList::default();
        value.into_iter().for_each(|face| list.add(Arc::new(face)));
        list
    }
}