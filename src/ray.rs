use std::ops::Mul;
use glm::Vec3;
use na::Rotation3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, distance: f32) -> Vec3 {
        self.origin + distance * self.direction
    }
}

impl Mul<Ray> for Rotation3<f32> {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray {
            origin: self * rhs.origin,
            direction: self * rhs.direction
        }
    }
}