/*module.exports = class Rect {
    constructor(pointA, pointB) {
        if (pointA.x < pointB.x) {
            this.left = pointA.x;
            this.right = pointB.x;
        } else {
            this.left = pointB.x;
            this.right = pointA.x;
        }
        if (pointA.y < pointB.y) {
            this.top = pointA.y;
            this.bottom = pointB.y;
        } else {
            this.top = pointB.y;
            this.bottom = pointA.y;
        }
    }
}*/
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
