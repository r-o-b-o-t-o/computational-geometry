use std::ops::{ Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Neg };

use rand::Rng;
use glium::vertex;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

unsafe impl vertex::Attribute for Vec2 {
    fn get_type() -> vertex::AttributeType {
        vertex::AttributeType::F32F32
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn is_zero(self) -> bool {
        self.x.abs() < 0.00001 &&
        self.y.abs() < 0.00001
    }

    const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl Vec2 {
    pub fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }

    pub fn sqr_length(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalize(&mut self) -> &mut Self {
        let len = self.length();
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
        self
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    pub fn clamp(&mut self, max_length: f32) -> &mut Self {
        if self.sqr_length() > max_length.powi(2) {
            self.normalize();
            *self *= max_length;
        }
        self
    }

    pub fn clamped(mut self, max_length: f32) -> Self {
        self.clamp(max_length);
        self
    }

    pub fn randomize(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.x = rng.gen();
        self.y = rng.gen();
        self
    }

    pub fn randomize_range(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(x_min, x_max);
        self.y = rng.gen_range(y_min, y_max);
        self
    }

    pub fn random() -> Self {
        let mut v = Self::default();
        v.randomize();
        v
    }

    pub fn random_range(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        let mut v = Self::default();
        v.randomize_range(x_min, x_max, y_min, y_max);
        v
    }

    /// Returns the dot product of two vectors A and B (xA * xB + yA * yB).
    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the signed angle in radians between -pi and pi.
    pub fn signed_angle(self, other: Vec2) -> f32 {
        (self.x * other.y - self.y * other.x).atan2(self.dot(other))
    }
}

impl Mul<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<&Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign<&Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: &Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for &Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl DivAssign<&Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: &Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for &Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(arr: [f32; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }
}

impl From<&[f32; 2]> for Vec2 {
    fn from(arr: &[f32; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }
}

impl Into<[f32; 2]> for &Vec2 {
    fn into(self) -> [f32; 2] {
        [ self.x, self.y ]
    }
}
