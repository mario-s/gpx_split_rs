use log::trace;
use geo_types::{coord, Rect};
use geo_types::Point as Geopoint;
use gpx::{Gpx, Waypoint};
use haversine_rs::distance_vec;
use haversine_rs::point::Point as Havpoint;
use haversine_rs::units::Unit;

/// Calculates the distance of all waypoints in the track.
/// Returns result in Meter.
///
pub fn distance_points(way_points: &[Waypoint]) -> f64 {
    distance(collect_points(way_points))
}

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Havpoint>) -> f64 {
    distance_vec(points, Unit::Meters)
}

/// This will adjust the bounds of the metadata, if they are set.
/// The new bounding box will a Rect which contains min/max of x/y.
///
pub fn fit_bounds(mut gpx: Gpx, way_points: &Vec<Waypoint>) -> Gpx {
    if let Some(existing) = gpx.metadata {
        let mut m = existing.clone();
        if m.bounds.is_some() {
            m.bounds = find_bounds(way_points);
        }
        gpx.metadata = Some(m);
    }
    gpx
}

/// Find the bounding box, min x, min y, max x, max y from the given way points.
///
fn find_bounds(way_points: &Vec<Waypoint>) -> Option<Rect<f64>> {
    if way_points.is_empty() {
        return None;
    }

    let points = collect_points(way_points);
    let min_x = points
        .iter()
        .map(|p| p.latitude)
        .fold(f64::INFINITY, f64::min);
    let max_x = points
        .iter()
        .map(|p| p.latitude)
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = points
        .iter()
        .map(|p| p.longitude)
        .fold(f64::INFINITY, f64::min);
    let max_y = points
        .iter()
        .map(|p| p.longitude)
        .fold(f64::NEG_INFINITY, f64::max);
    Some(Rect::new(
        coord! { x: min_x, y: min_y },
        coord! { x: max_x, y: max_y },
    ))
}

/// Collect the points (x, y) from the given way points
///
fn collect_points(way_points: &[Waypoint]) -> Vec<Havpoint> {
    way_points
        .iter()
        .map(|p| p.point())
        .map(|p| Havpoint::new(p.x(), p.y()))
        .collect()
}

fn _is_point_on_line(point: Waypoint, line: (Waypoint, Waypoint)) -> bool {
    // get points for easy calculation
    let point = point.point();
    let start = line.0.point();
    let end = line.1.point();

    // calculate vectors
    let vec_1 = Geopoint::new(point.x() - start.x(), point.y() - start.y());
    let vec_2 = Geopoint::new(end.x() - start.x(), end.y() - start.y());

    // check if the vectors are collinear (their cross product is close to zero)
    let cross_product = vec_2.x() * vec_1.y() - vec_2.y() * vec_1.x();
    trace!("cross product of vectors: {}", cross_product);
    cross_product.abs() < 1e-10
}

#[cfg(test)]
mod tests {
    use geo_types::Point as Geopoint;
    use geo_types::{coord, Rect};
    use gpx::{Gpx, Metadata, Waypoint};
    use haversine_rs::point::Point as Havpoint;

    use crate::geo::{distance, find_bounds, fit_bounds, _is_point_on_line};

    fn waypoint(x: f64, y: f64) -> Waypoint {
        Waypoint::new(Geopoint::new(x, y))
    }

    #[test]
    fn test_distance() {
        let point_0 = Havpoint::new(40.7767644, -73.9761399);
        let point_1 = Havpoint::new(40.771209, -73.9673991);
        let distance = distance(vec![point_0, point_1]);
        assert!(distance == 960.9072987659282);
    }

    #[test]
    fn test_fit_bounds() {
        let mut meta = Metadata::default();
        let rect = Rect::new(coord! { x: 10., y: 20. }, coord! { x: 30., y: 10. });
        meta.bounds = Some(rect);
        let mut gpx = Gpx::default();
        let gpx_ref = &mut gpx;
        gpx_ref.metadata = Some(meta);

        let res = fit_bounds(gpx_ref.clone(), &vec![]);
        assert_eq!(None, res.metadata.and_then(|m| m.bounds))
    }

    #[test]
    fn test_find_bounding_box() {
        let points = vec![waypoint(40.7767644, -73.9761399), waypoint(40.771209, -73.9673991)];
        let rect = find_bounds(&points).unwrap();
        assert_eq!(40.771209, rect.min().x);
        assert_eq!(-73.9761399, rect.min().y);
        assert_eq!(40.7767644, rect.max().x);
        assert_eq!(-73.9673991, rect.max().y);
    }

    #[test]
    fn test_is_point_on_line_true() {
        assert!(_is_point_on_line(waypoint(2.0, 2.0), (waypoint(2.0, 2.0), waypoint(4.0, 4.0))));
        assert!(_is_point_on_line(waypoint(3.0, 3.0), (waypoint(2.0, 2.0), waypoint(4.0, 4.0))));
        assert!(_is_point_on_line(waypoint(40.8, -73.8), (waypoint(40.9, -73.9), waypoint(40.7, -73.7))));
    }

    #[test]
    fn test_is_point_on_line_false() {
        assert!(!_is_point_on_line(waypoint(1.0, 2.0), (waypoint(2.0, 2.0), waypoint(4.0, 4.0))));
        assert!(!_is_point_on_line(waypoint(3.0, 2.0), (waypoint(2.0, 2.0), waypoint(4.0, 4.0))));
        assert!(!_is_point_on_line(waypoint(41.8, -73.8), (waypoint(40.9, -73.9), waypoint(40.7, -73.7))));
    }
}
