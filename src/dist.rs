use gpx::Waypoint;

use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point>) -> f64 {
    distance_vec(points, Unit::Meters)
}

#[test]
fn test_distance() {
    let point_0 = Point::new(40.7767644, -73.9761399);
    let point_1 = Point::new(40.771209, -73.9673991);
    let distance = distance(vec![point_0, point_1]);
    assert!(distance == 960.9072987659282);
}

/// Calculates the distance of all waypoints in the track.
/// Returns result in Meter.
pub fn distance_points(way_points: Vec<Waypoint>) -> f64 {
    let points = way_points.iter().map(|p| p.point()).map(|p| Point::new(p.x(), p.y())).collect();
    distance(points)
}