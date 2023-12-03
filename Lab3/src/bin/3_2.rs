// // use std::f64;

// #[derive(Debug, PartialEq)]
// enum Location {
//     Inside,
//     Outside,
// }

// #[derive(Debug, PartialEq)]
// struct PointLocation {
//     // location: Location,
//     // distance: f64,
// }

// struct Circle {
//     center: Point,
//     radius: f64,
// }

// struct Point {
//     x: f64,
//     y: f64,
// }

// fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
//     ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
// }

// fn locate_n_point(c: &Circle, pt_list: &[Point]) -> Vec<PointLocation> {
//     let mut result = Vec::new();

//     for point in pt_list {
//         let distance = distance_between_points(&point, &c.center);
//         let location = if distance < c.radius {
//             Location::Inside
//         } else {
//             Location::Outside
//         };

//         result.push(PointLocation {
//             location,
//             distance,
//         });
//     }

//     result
// }

// #[test]
// fn test_locate_n_point() {
//     let c = Circle {
//         center: Point { x: 0.0, y: 0.0 },
//         radius: 1.0,
//     };

//     let pt_list = vec![
//         Point { x: 0.5, y: 0.5 },
//         Point { x: 1.5, y: 1.5 },
//         Point { x: 0.0, y: 0.0 },
//         Point { x: -1.0, y: -1.0 },
//     ];

//     let result = locate_n_point(&c, &pt_list);

//     let expected_result = vec![
//         PointLocation {
//             location: Location::Inside,
//             distance: 0.7071067811865476, // Approximate value
//         },
//         PointLocation {
//             location: Location::Outside,
//             distance: 2.1213203435596424, // Approximate value
//         },
//         PointLocation {
//             location: Location::Inside,
//             distance: 0.0,
//         },
//         PointLocation {
//             location: Location::Outside,
//             distance: 1.4142135623730951, // Approximate value
//         },
//     ];

//     assert_eq!(result, expected_result);
// }
