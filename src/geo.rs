use gpx::{Gpx, Waypoint};

use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point>) -> f64 {
    distance_vec(points, Unit::Meters)
}

/// Calculates the distance of all waypoints in the track.
/// Returns result in Meter.
pub fn distance_points(way_points: Vec<Waypoint>) -> f64 {
    let points = way_points.iter().map(|p| p.point()).map(|p| Point::new(p.x(), p.y())).collect();
    distance(points)
}

pub fn adjust_bounds(mut gpx: Gpx) -> Gpx {
    if let Some(existing) = gpx.metadata {
        let mut m = existing.clone();
        if let Some(_) = m.bounds {
            //TODO find min, max from current trace
            m.bounds = None;
        }
        gpx.metadata = Some(m);
    }
    gpx
}

#[cfg(test)]
mod tests {
    use gpx::{Gpx, Metadata};
    use geo_types::{coord, Rect};
    use haversine_rs::point::Point;

    use crate::geo::{adjust_bounds, distance};

    #[test]
    fn test_adjust_bounds() {
        let mut meta = Metadata::default();
        let rect = Rect::new(
                coord! { x: 10., y: 20. },
                coord! { x: 30., y: 10. }
            );
        meta.bounds = Some(rect);
        let mut gpx = Gpx::default();
        gpx.metadata = Some(meta);

        let res = adjust_bounds(gpx);
        assert_eq!(None, res.metadata.and_then(|m| m.bounds))
    }

    #[test]
    fn test_distance() {
        let point_0 = Point::new(40.7767644, -73.9761399);
        let point_1 = Point::new(40.771209, -73.9673991);
        let distance = distance(vec![point_0, point_1]);
        assert!(distance == 960.9072987659282);
    }
}
