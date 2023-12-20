use geo_types::{coord, Rect};
use gpx::{Gpx, Waypoint};
use geo::prelude::*;
use geo::Point;
use geo::point;

/// Calculates the distance of all waypoints in the track.
/// Returns result in Meter.
///
pub fn distance_points(way_points: &[Waypoint]) -> f64 {
    distance(collect_points(way_points))
}

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point<f64>>) -> f64 {
    points
    .iter().zip(points.iter().skip(1))
    .map(|(&p1, &p2)| p1.geodesic_distance(&p2))
    .sum()
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
        .map(|p| p.y())
        .fold(f64::INFINITY, f64::min);
    let max_x = points
        .iter()
        .map(|p| p.y())
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = points
        .iter()
        .map(|p| p.x())
        .fold(f64::INFINITY, f64::min);
    let max_y = points
        .iter()
        .map(|p| p.x())
        .fold(f64::NEG_INFINITY, f64::max);
    Some(Rect::new(
        coord! { x: min_x, y: min_y },
        coord! { x: max_x, y: max_y },
    ))
}

/// Collect the points (x, y) from the given way points
///
fn collect_points(way_points: &[Waypoint]) -> Vec<Point<f64>> {
    way_points
        .iter()
        .map(|p| p.point())
        .map(|p| point!(x: p.x(), y: p.y()))
        .collect()
}

/// Calculate the distance from a point to the geodesic in meter.
/// A straight line between two points, on the earth surface is a geodesic.
/// The closest point on a geodesic to another point, is referred to as the interception point.
fn _distance_to_line(point: Waypoint, geodesic: (Waypoint, Waypoint)) -> f64 {
    let points = collect_points(&vec![geodesic.0, geodesic.1, point]);

    let geodesic_len = distance(points[0..2].to_vec());
    let d_p1_p3 = distance(vec![points[1], points[2]]);
    let d_p2_p3 = distance(points[1..3].to_vec());

    let s = (geodesic_len + d_p1_p3 + d_p2_p3) / 2.0;
    let area = (s * (s - geodesic_len) * (s - d_p1_p3) * (s - d_p2_p3)).sqrt();

    (2.0 * area / geodesic_len).abs()
}

#[cfg(test)]
mod tests {
    use geo_types::Point as Geopoint;
    use geo_types::{coord, Rect};
    use gpx::{Gpx, Metadata, Waypoint};
    use approx_eq::assert_approx_eq;

    use crate::geo::{find_bounds, fit_bounds, _distance_to_line, distance_points};

    fn waypoint(x: f64, y: f64) -> Waypoint {
        Waypoint::new(Geopoint::new(x, y))
    }

    #[test]
    fn test_distance() {
        let point_0 = waypoint(-73.9761399, 40.7767644);
        let point_1 = waypoint(-73.9673991, 40.771209,);
        let distance = distance_points(&[point_0, point_1]);
        assert_approx_eq!(distance, 961.8288);
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
        let points = vec![waypoint(-73.9761399, 40.7767644), waypoint(-73.9673991, 40.771209)];
        let rect = find_bounds(&points).unwrap();
        assert_eq!(40.771209, rect.min().x);
        assert_eq!(-73.9761399, rect.min().y);
        assert_eq!(40.7767644, rect.max().x);
        assert_eq!(-73.9673991, rect.max().y);
    }

    #[test]
    fn test_distance_to_line() {
        //0.00028° = 0°0'1" ~ 30.9 m
        let d = distance_points(&[waypoint(0.0, 0.00028), waypoint(0.0, 0.0)]);
        let res = _distance_to_line(waypoint(0.0, 0.00028), (waypoint(-0.005, 0.0), waypoint(0.005, 0.0)));
        assert_approx_eq!(d, res, 1e-8);
    }
}
