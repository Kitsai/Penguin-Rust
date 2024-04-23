#[cfg(test)]
mod tests;

use crate::arith::vec3::Vec3;

struct Rect {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32
}

impl Rect {
    pub fn new(x: f32, y: f32, z: f32, w: f32, h:f32) -> Self {
        Rect {
            x,
            y,
            z,
            w,
            h
        }
    }

    pub fn area(&self) -> f32{
        return self.w * self.h;
    }

    pub fn center(&self) -> Vec3 {
        return Vec3 {x: self.x + (self.w*0.5f32), y: self.y + (self.h*0.5f32), z: self.z}
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        self.x = x - (self.w*0.5f32);
        self.y = y - (self.h*0.5f32);
    }

    pub fn get_pos(&self) -> Vec3 {
        return Vec3 {x: self.x, y: self.y, z: self.z}
    }
}