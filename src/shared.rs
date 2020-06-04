use nalgebra::{Vector2, Vector3};

pub const BIT_MASK_N: u64 = 0b11;
pub const BIT_MASK_S: u64 = 0b10;
pub const BIT_MASK_0: u64 = 0b00;
pub const BIT_MASK_1: u64 = 0b01;
pub const BIT_MASK_2: u64 = 0b10;
pub const BIT_MASK_3: u64 = 0b11;

pub trait HtmIndex {
    fn build(max_depth: u8) -> Self;
}

pub fn vec2(x: f32, y: f32) -> Vector2<f32> {
    Vector2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}
