#[cfg(test)]
use super::*;

#[test]
fn vec2_is_zero() {
    let mut v = Vec2::new(0.0, 0.0);
    assert!(v.is_zero());

    v = Vec2::new(4.0, 0.0);
    assert!(!v.is_zero());

    v = Vec2::default();
    assert!(v.is_zero());
}

#[test]
fn vec2_length() {
    let mut v = Vec2::new(6.0, 3.0);
    assert!(cmp_f32(v.sqr_length(), 45.0));
    assert!(cmp_f32(v.length(), 6.708203933));

    v = Vec2::new(-4.0, 2.0);
    assert!(cmp_f32(v.sqr_length(), 20.0));
    assert!(cmp_f32(v.length(), 4.472135955));

    v = Vec2::new(0.0, -5.0);
    assert!(cmp_f32(v.sqr_length(), 25.0));
    assert!(cmp_f32(v.length(), 5.0));
}

#[test]
fn vec2_normalize() {
    let mut v = Vec2::new(4.0, -7.0);
    assert!(cmp_f32(v.normalized().length(), 1.0));

    v = Vec2::new(-8.0, 6.0);
    let n = v.normalized();
    assert_eq!(n, Vec2::new(-0.8, 0.6));

    v.normalize();
    assert_eq!(n, v);
}

#[test]
fn vec2_clamp() {
    let mut v = Vec2::new(4.0, 10.0);
    v.clamp(5.0);
    assert!(cmp_f32(v.length(), 5.0));

    v  = Vec2::new(4.0, 10.0);
    let clamped = v.clamped(15.0);
    assert!(cmp_f32(v.length(), clamped.length()));

    v = Vec2::new(24.0, 18.0);
    v.clamp(2.0);
    assert_eq!(v, Vec2::new(1.6, 1.2));
}

#[test]
fn vec2_slope() {
    let mut v = Vec2::new(4.0, 2.0);
    assert!(cmp_f32(v.slope(), 0.5));

    v = Vec2::new(2.0, -8.0);
    assert!(cmp_f32(v.slope(), -4.0));

    v = Vec2::new(6.0, 0.0);
    assert!(cmp_f32(v.slope(), 0.0));

    v = Vec2::new(-1.0, 0.5);
    assert!(cmp_f32(v.slope(), -0.5));
}

#[test]
fn vec2_collinearity() {
    let a = Vec2::new(8.0, -4.0);
    let b = Vec2::new(-2.0, 1.0);
    assert!(a.collinear(b));
}

#[test]
fn vec2_dot() {
    let a = Vec2::new(2.0, 5.0);
    let b = Vec2::new(7.0, -1.0);
    assert!(cmp_f32(a.dot(b), 9.0));
}

#[test]
fn vec2_signed_angle() {
    let mut a = Vec2::new(6.0, 0.0);
    let mut b = Vec2::new(0.0, 1.5);
    let mut angle = a.signed_angle(b);
    assert!(cmp_f32(angle, std::f32::consts::FRAC_PI_2));

    a = Vec2::new(0.0, -8.0);
    b = Vec2::new(-2.0, -2.0);
    angle = a.signed_angle(b);
    assert!(cmp_f32(angle, -std::f32::consts::FRAC_PI_4));
}

#[test]
fn vec2_cw() {
    let mut a = Vec2::new(0.0, 8.0);
    let mut b = Vec2::new(2.0, -1.0);
    let mut c = Vec2::new(1.0, -5.0);
    assert!(Vec2::cw(a, b, c));

    a = Vec2::new(-2.0, -1.0);
    b = Vec2::new(4.0, 1.0);
    c = Vec2::new(-3.0, 2.0);
    assert!(Vec2::ccw(a, b, c));
}

#[test]
fn vec2_shoelace() {
    let a = Vec2::new(3.0, 4.5);
    let b = Vec2::new(-2.0, 0.25);
    let c = Vec2::new(8.0, -3.75);
    assert!(cmp_f32(Vec2::shoelace(a, b, c), 62.5));
}

#[test]
fn segment2_degenerate() {
    let s = Segment2::new(Vec2::new(-4.0, 0.0), Vec2::new(2.0, -1.0));
    assert!(!s.is_degenerate());

    let s = Segment2::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
    assert!(s.is_degenerate());
}

#[test]
fn segment2_bounds() {
    let s = Segment2::new(Vec2::new(-4.0, 0.0), Vec2::new(2.0, -1.0));
    let r = s.bounding_rect();
    assert!(cmp_f32(r.left, -4.0));
    assert!(cmp_f32(r.right, 2.0));
    assert!(cmp_f32(r.top, -1.0));
    assert!(cmp_f32(r.bottom, 0.0));
}

#[test]
fn segment2_contains() {
    let s = Segment2::new(Vec2::new(8.0, 2.0), Vec2::new(4.0, 0.0));
    assert!(s.contains(Vec2::new(6.0, 1.0)));
    assert!(s.contains(Vec2::new(7.5, 1.75)));
    assert!(s.contains(Vec2::new(5.0, 0.5)));
    assert!(!s.contains(Vec2::new(6.0, 0.6)));
}

#[test]
fn segment2_intersections() {
    let s1 = Segment2::new(Vec2::new(8.0, 2.0), Vec2::new(4.0, 0.0));
    let s2 = Segment2::new(Vec2::new(2.0, 2.0), Vec2::new(6.0, -0.5));
    let s3 = Segment2::new(Vec2::new(3.0, 3.0), Vec2::new(3.0, -4.0));

    assert!(s1.intersects(&s2));
    assert!(s2.intersects(&s3));
    assert!(!s1.intersects(&s3));

    assert_eq!(s1.intersection(&s2), Vec2::new(4.666666666, 0.333333333));
    assert_eq!(s2.intersection(&s3), Vec2::new(3.0, 1.375));
}

#[test]
fn segment2_y_intercept() {
    let s = Segment2::new(Vec2::new(3.0, 3.0), Vec2::new(3.0, -4.0));
    assert!(s.y_intercept().is_nan());

    let s = Segment2::new(Vec2::new(8.0, 2.0), Vec2::new(4.0, 0.0));
    assert!(cmp_f32(s.y_intercept(), -2.0));
}
