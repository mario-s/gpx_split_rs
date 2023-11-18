use gpx::{Gpx, Waypoint};
use geo_types::{coord, Rect};
use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

/// Calculates the distance of all waypoints in the track.
/// Returns result in Meter.
///
pub fn distance_points(way_points: &[Waypoint]) -> f64 {
    distance(collect_points(way_points))
}

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point>) -> f64 {
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
    let min_x = points.iter().map(|p| p.latitude).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.latitude).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.longitude).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.longitude).fold(f64::NEG_INFINITY, f64::max);
    Some(Rect::new(
        coord! { x: min_x, y: min_y },
        coord! { x: max_x, y: max_y }
    ))
}

/// Collect the points (x, y) from the given way points
///
fn collect_points(way_points: &[Waypoint]) -> Vec<Point> {
    way_points.iter().map(|p| p.point()).map(|p| Point::new(p.x(), p.y())).collect()
}

#[cfg(test)]
mod tests {
    use gpx::{Gpx, Metadata, Waypoint};
    use geo_types::{coord, Rect};
    use geo_types::Point as GeoPoint;
    use haversine_rs::point::Point as HavPoint;

    use crate::geo::{fit_bounds, distance, find_bounds};

    #[test]
    fn test_distance() {
        let point_0 = HavPoint::new(40.7767644, -73.9761399);
        let point_1 = HavPoint::new(40.771209, -73.9673991);
        let distance = distance(vec![point_0, point_1]);
        assert!(distance == 960.9072987659282);
    }

    #[test]
    fn test_fit_bounds() {
        let mut meta = Metadata::default();
        let rect = Rect::new(
                coord! { x: 10., y: 20. },
                coord! { x: 30., y: 10. }
            );
        meta.bounds = Some(rect);
        let mut gpx = Gpx::default();
        let gpx_ref = &mut gpx;
        gpx_ref.metadata = Some(meta);

        let res = fit_bounds(gpx_ref.to_owned(), &vec![]);
        assert_eq!(None, res.metadata.and_then(|m| m.bounds))
    }

    #[test]
    fn test_find_bounding_box() {
        let point_0 = GeoPoint::new(40.7767644, -73.9761399);
        let point_1 = GeoPoint::new(40.771209, -73.9673991);
        let points = vec![Waypoint::new(point_0), Waypoint::new(point_1)];
        let rect = find_bounds(&points).unwrap();
        assert_eq!(40.771209, rect.min().x);
        assert_eq!(-73.9761399, rect.min().y);
        assert_eq!(40.7767644, rect.max().x);
        assert_eq!(-73.9673991, rect.max().y);
    }
}
