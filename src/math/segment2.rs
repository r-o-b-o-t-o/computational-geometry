use super::{ Vec2, Rect, cmp_f32 };

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Segment2 {
    pub a: Vec2,
    pub b: Vec2,
}

impl Segment2 {
    /// Creates a segment [a, b]
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            a,
            b,
        }
    }

    /// Returns a vector that describes the direction of the segment
    pub fn as_vec2(&self) -> Vec2 {
        &self.b - &self.a
    }

    /// Returns true if the segment has no length
    pub fn is_degenerate(&self) -> bool {
        cmp_f32(self.a.x - self.b.x, 0.0) &&
        cmp_f32(self.a.y - self.b.y, 0.0)
    }

    /// Returns the rectangle that contains the segment
    pub fn bounding_rect(&self) -> Rect {
        Rect::new(self.a, self.b)
    }

    /// Returns whether the segment contains a given point
    pub fn contains(&self, point: Vec2) -> bool {
        if self.is_degenerate() {
            return false;
        }

        let bounds = self.bounding_rect();
        // Check if the point is contained in the segment's bounding rectangle
        if point.x < bounds.left || point.x > bounds.right ||
            point.y < bounds.top || point.y > bounds.bottom {
            return false;
        }

        // Check if the point is on the line formed by the extended segment
        let v = self.as_vec2();
        let a = v.slope();
        let b = v.y_intercept();
        cmp_f32(a * point.x + b, point.y)
    }

    /// Returns whether the two segments intersect
    pub fn intersects(&self, other: &Self) -> bool {
        if self.is_degenerate() || other.is_degenerate() {
            return false;
        }

        let v1 = self.as_vec2();
        let v2 = other.as_vec2();
        if v1.collinear(v2) {
            return self.contains(other.a) || self.contains(other.b);
        }

        let intersection = self.intersection(other);
        self.contains(intersection) && other.contains(intersection)
    }

    /// Returns the intersection point of two segments
    pub fn intersection(&self, other: &Self) -> Vec2 {
        let v1 = self.as_vec2();
        let v2 = other.as_vec2();

        if !v1.intersects(v2) {
            return Vec2::new(std::f32::NAN, std::f32::NAN);
        }

        let a1 = v1.slope();
        let a2 = v2.slope();

        // Special case if one of the lines is vertical
        if cmp_f32(v1.x, 0.0) && !cmp_f32(v2.x, 0.0) {
            let x = self.a.x;
            let y = a2 * (x - other.a.x) + other.a.y;
            return Vec2::new(x, y);
        }
        if cmp_f32(v2.x, 0.0) && !cmp_f32(v1.x, 0.0) {
            let x = other.a.x;
            let y = a1 * (x - self.a.x) + self.a.y;
            return Vec2::new(x, y);
        }

        let b1 = v1.y_intercept();
        let b2 = v2.y_intercept();
        let x = (b2 - b1) / (a1 - a2);
        let y = a1 * x + b1;
        Vec2::new(x, y)
    }
}
