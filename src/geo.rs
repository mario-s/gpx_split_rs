use haversine_rs::point::Point;
use haversine_rs::units::Unit;

/// Calculates the distance between two points.
/// Returns result in Meter.
///
pub fn distance(start: Point, end: Point) -> f64 {
    haversine_rs::distance(start, end, Unit::Meters)
}

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
pub fn distance_vec(points: Vec<Point>) -> f64 {
    haversine_rs::distance_vec(points, Unit::Meters)
}

#[test]
fn test_distance_vec() {
    let point_0 = Point::new(40.7767644, -73.9761399);
    let point_1 = Point::new(40.771209, -73.9673991);
    let distance = distance_vec(vec![point_0, point_1]);
    assert!(distance == 960.9072987659282);
}