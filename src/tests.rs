#[cfg(test)]
use crate::{
    math::Vec2,
    algorithms::*,
};

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
    let hull = JarvisMarch::march(points.iter());
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
    let hull = GrahamScan::scan(&points);
    println!("Graham scan: {}µs", timer.elapsed().as_micros());
    assert_eq!(hull, hull_expected);
}

#[test]
fn incremental_2d_triangulation() {
    let mut points = vec![
        Vec2::new(-0.63671875, -0.140625),
        Vec2::new(-0.47265625, 0.44791666),
        Vec2::new(-0.1484375, 0.7135417),
        Vec2::new(-0.29296875, 0.106770836),
        Vec2::new(0.017578125, 0.27864584),
        Vec2::new(0.30664063, 0.42447916),
        Vec2::new(0.5488281, -0.15364583),
        Vec2::new(-0.29492188, -0.33072916),
        Vec2::new(0.1796875, -0.33072916),
        Vec2::new(-0.013671875, -0.088541664),
        Vec2::new(0.1953125, -0.71875),
        Vec2::new(-0.15820313, -0.8307292),
    ];

    let timer = Instant::now();
    let indices = Incremental2dTriangulation::triangulate(&mut points);
    println!("Incremental basic 2D triangulation: {}µs", timer.elapsed().as_micros());
    let expected = vec![
        0, 1, 2,
        2, 1, 3,
        2, 3, 4,
        0, 2, 4,
        3, 1, 5,
        4, 3, 5,
        4, 5, 6,
        6, 5, 7,
        6, 7, 8,
        4, 6, 8,
        4, 8, 9,
        7, 5, 10,
        8, 7, 10,
        9, 8, 10,
        9, 10, 11,
    ];
    assert_eq!(indices, expected);
}
