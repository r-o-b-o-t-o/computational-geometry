pub mod vec2;
pub use vec2::Vec2;

pub mod segment2;
pub use segment2::Segment2;

pub mod rect;
pub use rect::Rect;


pub mod tests;

pub fn cmp_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.00001
}
