pub mod vec2;
pub use vec2::Vec2;

pub mod vec3;
pub use vec3::Vec3;

pub mod segment2;
pub use segment2::Segment2;

pub mod rect;
pub use rect::Rect;


pub mod tests;

pub fn cmp_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.00001
}

pub fn clamp(mut x: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    if x < min { x = min; }
    if x > max { x = max; }
    x
}

pub trait ToArray {
    type Output;
    fn to_array(&self) -> Self::Output;
}

impl<T: cgmath::BaseNum> ToArray for cgmath::Matrix4<T> {
    type Output = [[T; 4]; 4];
    fn to_array(&self) -> Self::Output {
        (*self).into()
    }
}
