#[cfg(test)]
use crate::math::Vec2;

#[test]
fn jarvis_march_basic() {
    let points = vec![
        Vec2::new(0.1328125, 0.2265625),
        Vec2::new(-0.123046875, 0.080729164),
        Vec2::new(0.26953125, 0.45833334), // 3
        Vec2::new(0.15429688, 0.390625),
        Vec2::new(0.001953125, 0.2890625),
        Vec2::new(-0.119140625, 0.38802084),
        Vec2::new(-0.1484375, -0.015625), // 5
        Vec2::new(-0.203125, 0.20833333),
        Vec2::new(0.1953125, 0.020833334), // 4
        Vec2::new(0.001953125, 0.1484375),
        Vec2::new(-0.2421875, 0.47135416), // 2
        Vec2::new(-0.34375, 0.17447917), // 1
    ];
    let hull_indices = [ 11, 10, 2, 8, 6 ];

    let hull = crate::algorithms::JarvisMarch::march(points.iter());
    for (idx, &point) in hull.iter().enumerate() {
        assert_eq!(point, points[hull_indices[idx]]);
    }
}
