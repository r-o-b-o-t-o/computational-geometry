use super::Vec2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    /// Creates a rectangle delimited by two corners
    pub fn new(point_a: Vec2, point_b: Vec2) -> Self {
        let (left, right) = if point_a.x < point_b.x {
            (point_a.x, point_b.x)
        } else {
            (point_b.x, point_a.x)
        };
        let (top, bottom) = if point_a.y < point_b.y {
            (point_a.y, point_b.y)
        } else {
            (point_b.y, point_a.y)
        };

        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}
