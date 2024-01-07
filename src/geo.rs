use geo_types::{coord, Rect};
use geo_types::Point as Geopoint;
use gpx::{Gpx, Waypoint};
use geo::prelude::*;
use geo::Point;
use geo::point;

/// Calculates the distance between the 2 waypoints.
/// Returns result in Meter.
///
pub fn distance(p1: &Waypoint, p2: &Waypoint) -> f64 {
    let point = |p: &Waypoint| {point!(x: p.point().x(), y: p.point().y())};
    let p1 = point(p1);
    let p2 = point(p2);
    p1.geodesic_distance(&p2)
}

/// Calculates the distance of all waypoints in the collection.
/// Returns result in Meter.
///
pub fn distance_all(points: &[Waypoint]) -> f64 {
    let points = collect_points(points);
    points
    .iter().zip(points.iter().skip(1))
    .map(|(&p1, &p2)| p1.geodesic_distance(&p2))
    .sum()
}

/// This will adjust the bounds of the metadata, if they are set.
/// The new bounding box is a rectangle which contains min/max of x/y.
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
fn collect_points(points: &[Waypoint]) -> Vec<Point<f64>> {
    points
        .iter()
        .map(|p| p.point())
        .map(|p| point!(x: p.x(), y: p.y()))
        .collect()
}

/// A straight line between two points, on the earth surface is a geodesic.
/// The closest point on a geodesic to another point, is referred to as the interception point.
/// ```
/// use gpx::Waypoint;
/// use geo::Point;
/// use gpx_split::prelude::*;
/// use approx_eq::assert_approx_eq;
///
/// let p = Waypoint::new(Point::new(0.0, 1.0));
/// let ip = interception_point(&p, (&Waypoint::new(Point::new(-1.0, 0.0)), &Waypoint::new(Point::new(1.0, 0.0))));
/// assert_approx_eq!(0.4094528, ip.point().x());
/// assert_approx_eq!(0.0, ip.point().y());
/// ```
pub fn interception_point(point: &Waypoint, geodesic: (&Waypoint, &Waypoint)) -> Waypoint {
    let p1 = geodesic.0.point();
    let p1 = point!(x: p1.x(), y: p1.y());
    let p2 = geodesic.1.point();
    let p2 = point!(x: p2.x(), y: p2.y());
    let p3 = point.point();
    let p3 = point!(x: p3.x(), y: p3.y());

    // Calculate bearing from p1 to p2
    let bearing = p1.geodesic_bearing(p2);

    // Calculate geodesic distance from p1 to p3
    let distance = p1.geodesic_distance(&p3);

    // Calculate the interception point from p1 in the direction of p2 with the distance
    let interception = p1.geodesic_destination(bearing, distance);
    Waypoint::new(Geopoint::new(interception.x(), interception.y()))
}

#[cfg(test)]
mod tests {

    use geo_types::{coord, Rect, Point};
    use gpx::{Gpx, Metadata, Waypoint};
    use approx_eq::assert_approx_eq;

    use super::*;

    fn waypoint(x: f64, y: f64) -> Waypoint {
        Waypoint::new(Point::new(x, y))
    }

    #[test]
    fn distance_2() {
        let point_0 = waypoint(-73.9761399, 40.7767644);
        let point_1 = waypoint(-73.9673991, 40.771209,);
        let d = distance(&point_0, &point_1);
        assert_approx_eq!(d, 961.8288);
    }

    #[test]
    fn distance_array() {
        let point_0 = waypoint(-73.9761399, 40.7767644);
        let point_1 = waypoint(-73.9673991, 40.771209,);
        let distance = distance_all(&[point_0, point_1]);
        assert_approx_eq!(distance, 961.8288);
    }

    #[test]
    fn no_waypoints_no_bounds() {
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
    fn find_bounding_box() {
        let points = vec![waypoint(-73.9761399, 40.7767644), waypoint(-73.9673991, 40.771209)];
        let rect = find_bounds(&points).unwrap();
        assert_eq!(40.771209, rect.min().x);
        assert_eq!(-73.9761399, rect.min().y);
        assert_eq!(40.7767644, rect.max().x);
        assert_eq!(-73.9673991, rect.max().y);
    }

    #[test]
    fn distance_to_line() {
        //0.00028° = 0°0'1" ~ 30.9 m
        let p = waypoint(0.0, 0.00028);
        let ip = interception_point(&p, (&waypoint(-1.0, 0.0), &waypoint(1.0, 0.0)));
        let dist_p_ip = distance(&p, &ip);
        assert_approx_eq!(30.9607975, dist_p_ip);
    }
}
