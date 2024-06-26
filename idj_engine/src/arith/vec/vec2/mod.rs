use std::ops::{Add, Sub, Mul};
use super::Vec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec for Vec2 {
    fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    fn normalized(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Vec2 {
   pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }


    pub fn inclination(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn inclination_vec2(&self, other: &Self) -> f32 {
        let diff = Self::new(self.x - other.x, self.y - other.y);
        diff.inclination()
    }

    pub fn get_rotated(&self, angle: f32) -> Self {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Self { x, y }
    }

    pub fn as_vec3(&self, z: f32) -> super::vec3::Vec3 {
        super::vec3::Vec3::new(self.x, self.y, z)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests;