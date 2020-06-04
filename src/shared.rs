use nalgebra::{Vector2, Vector3};

//pub type TriangleMap = BTreeMap<String, Triangle>;

pub trait HtmIndex {
    fn build(max_depth: u8) -> Self;
}

pub fn vec2(x: f32, y: f32) -> Vector2<f32> {
    Vector2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}
