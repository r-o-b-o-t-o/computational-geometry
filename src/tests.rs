#[cfg(test)]
use crate::math::Vec2;
#[cfg(test)]
use std::time::Instant;

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

    let timer = Instant::now();
    let hull = crate::algorithms::JarvisMarch::march(points.iter());
    println!("Jarvis march: {}µs", timer.elapsed().as_micros());
    for (idx, &hull_idx) in hull.iter().enumerate() {
        assert_eq!(hull_idx, hull_indices[idx]);
    }
}

#[test]
fn graham_scan_basic() {
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
    // inversed because it uses a stack
    let hull_expected = vec![ 
        points[6], // 5
        points[8], // 4
        points[2], // 3
        points[10], // 2
        points[11], // 1
    ];

    let timer = Instant::now();
    let hull = crate::algorithms::GrahamScan::scan(&points);
    println!("Graham scan: {}µs", timer.elapsed().as_micros());
    assert_eq!(hull, hull_expected);
}
