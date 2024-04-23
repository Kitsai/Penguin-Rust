pub trait Vec {
    fn sqr_magnitude(&self) -> f32;
    fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }
    fn normalized(&self) -> Self;
    fn abs(&self) -> Self;
}

pub mod vec2;
pub mod vec3;