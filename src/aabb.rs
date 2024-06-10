use std::cmp::Ordering;
use std::ops::{Add, Index};
use glm::Vec3;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        const DELTA: f32 = 0.0001;

        Self {
            x: x.expanded_if_smaller(DELTA),
            y: y.expanded_if_smaller(DELTA),
            z: z.expanded_if_smaller(DELTA),
        }
    }

    pub fn from_extrema(a: Vec3, b: Vec3) -> Self {
        Self::new(
            Interval::from_cmp(a.x ,b.x),
            Interval::from_cmp(a.y ,b.y),
            Interval::from_cmp(a.z ,b.z),
        )
    }

    pub fn enclosing(&self, other: &AABB) -> Self {
        Self::new(
            Interval::from_enclosing(&self.x, &other.x),
            Interval::from_enclosing(&self.y, &other.y),
            Interval::from_enclosing(&self.z, &other.z),
        )
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            n => panic!("Axis {n} was out of range [0,3)."),
        }
    }

    pub fn hit(&self, ray: Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);

            if ray.direction[axis].abs() < f32::EPSILON {
                if ray.origin[axis] < ax.min || ray.origin[axis] > ax.max {
                    return false;
                }
                continue;
            }

            let ad_inv = 1.0 / ray.direction[axis];

            let t0 = (ax.min - ray.origin[axis]) * ad_inv;
            let t1 = (ax.max - ray.origin[axis]) * ad_inv;

            ray_t.min = ray_t.min.max(t0.min(t1));
            ray_t.max = ray_t.max.min(t0.max(t1));

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn compare_by_axis(&self, other: &AABB, axis: usize) -> Ordering {
        self.axis_interval(axis).min.total_cmp(&other.axis_interval(axis).min)
    }

    pub fn axes(&self) -> [&Interval; 3] {
        [&self.x, &self.y, &self.z]
    }

    pub fn longest_axis(&self) -> usize {
        #[expect(clippy::collapsible_else_if, reason = "readability; both branches should look the same")]
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.x.size() > self.z.size() { 1 } else { 2 }
        }
    }
}

impl Index<usize> for AABB {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for AABB"),
        }
    }
}

impl Add<Vec3> for &AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        &self + rhs
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, rhs: AABB) -> Self::Output {
        rhs + self
    }
}