use geo::point;
use geo::prelude::*;
use geo::Point;
use geo_types::Point as Geopoint;
use geo_types::{coord, Rect};
use geographiclib_rs::{DirectGeodesic, Geodesic, InverseGeodesic};
use gpx::{Gpx, Waypoint};

/// Calculates the distance between the 2 waypoints.
/// Returns result in Meter.
///
#[must_use]
pub fn distance(p1: &Waypoint, p2: &Waypoint) -> f64 {
    let point = |p: &Waypoint| point!(x: p.point().x(), y: p.point().y());
    point(p1).geodesic_distance(&point(p2))
}

/// Calculates the distance of all waypoints in the collection.
/// Returns result in Meter.
///
pub fn distance_all(points: &[Waypoint]) -> f64 {
    let points = collect_points(points);
    points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(&p1, &p2)| p1.geodesic_distance(&p2))
        .sum()
}

/// This will adjust the bounds of the metadata, if they are set.
/// The new bounding box is a rectangle which contains min/max of x/y.
///
#[must_use]
pub fn fit_bounds(mut gpx: Gpx, way_points: &[Waypoint]) -> Gpx {
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
#[must_use]
fn find_bounds(way_points: &[Waypoint]) -> Option<Rect<f64>> {
    if way_points.is_empty() {
        return None;
    }

    let points = collect_points(way_points);
    let min_x = points.iter().map(|p| p.y()).fold(f64::INFINITY, f64::min);
    let max_x = points
        .iter()
        .map(|p| p.y())
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.x()).fold(f64::INFINITY, f64::min);
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
        .map(Waypoint::point)
        .map(|p| point!(x: p.x(), y: p.y()))
        .collect()
}

/// A straight line between two points on the earth's surface is a geodesic.
/// This function calculates the interception point, which is the closest point on a geodesic to another point.
///
/// ```
/// use gpx::Waypoint;
/// use geo::Point;
/// use gpx_split::loc::*;
/// use approx_eq::assert_approx_eq;
///
/// let p = Waypoint::new(Point::new(52.5186118, 13.408056));
/// let ip = intercept(&p, (&Waypoint::new(Point::new(64.15, -21.933333)), &Waypoint::new(Point::new(55.75, 37.616667))));
/// assert_approx_eq!(61.6561898, ip.point().x());
/// assert_approx_eq!(20.543903, ip.point().y());
/// ```
pub fn intercept(point: &Waypoint, geodesic: (&Waypoint, &Waypoint)) -> Waypoint {
    let geod = Geodesic::wgs84();
    //equatorial radius
    let radius: f64 = geod.a;

    let point_n = (point.point().x(), point.point().y());
    let mut point_a = (geodesic.0.point().x(), geodesic.0.point().y());
    let point_b = (geodesic.1.point().x(), geodesic.1.point().y());

    loop {
        let a_n: (f64, f64, f64, f64) = geod.inverse(point_a.0, point_a.1, point_n.0, point_n.1);
        let a_b: (f64, f64, f64, f64) = geod.inverse(point_a.0, point_a.1, point_b.0, point_b.1);
        //distance p_a to p_n
        let dist_ap = a_n.0;
        //azimuth
        let azi: f64 = a_n.1 - a_b.1;

        let s_px: f64 = radius * ((dist_ap / radius).sin() * azi.to_radians().sin()).asin();
        let s_ax: f64 = 2.0
            * radius
            * (((90.0 + azi) / 2.0).to_radians().sin() / ((90.0 - azi) / 2.0).to_radians().sin()
                * ((dist_ap - s_px) / (2.0 * radius)).tan())
            .atan();

        let p_a2: (f64, f64, f64, f64) = geod.direct(point_a.0, point_a.1, a_b.1, s_ax);

        if s_ax.abs() < 1e-2 {
            break;
        }

        point_a = (p_a2.0, p_a2.1);
    }
    Waypoint::new(Geopoint::new(point_a.0, point_a.1))
}

/// Returns true if the point is on the segment or behind one of the endpoints of the segement
/// with an allowed maximum distance, otherwise false.
/// The parameter max is the maximum distance allowed to be considered as "near" in meter.<br/>
/// ```text
/// ---  line
///  )   max
///  +   point
/// |-|  segment
///
/// |-------------|---+-----)--- => true
/// |-------+-----|---------)--- => true
/// |-------------|---------)-+- => false
/// ```
/// Example
/// ```
/// use gpx::Waypoint;
/// use geo::Point;
/// use gpx_split::loc::*;
///
/// let point = Waypoint::new(Point::new(0.0, 1.0));
/// let segment = (&Waypoint::new(Point::new(0.0, 0.5)), &Waypoint::new(Point::new(0.0, 1.0)));
/// assert!(is_near_segment(&point, segment, 0.1));
/// ```
#[must_use]
pub fn is_near_segment(point: &Waypoint, segment: (&Waypoint, &Waypoint), max: f64) -> bool {
    let dist_seg = distance(segment.0, segment.1);
    let dist_start_point = distance(segment.0, point);
    let dist_end_point = distance(segment.1, point);
    let d = (dist_start_point + dist_end_point - dist_seg).abs();
    d < max
}

#[cfg(test)]
mod tests {

    use approx_eq::assert_approx_eq;
    use geo_types::{coord, Point, Rect};
    use gpx::{Gpx, Metadata, Waypoint};

    use super::*;

    fn waypoint(x: f64, y: f64) -> Waypoint {
        Waypoint::new(Point::new(x, y))
    }

    #[test]
    fn distance_2() {
        let point_0 = waypoint(-73.9761399, 40.7767644);
        let point_1 = waypoint(-73.9673991, 40.771209);
        let d = distance(&point_0, &point_1);
        assert_approx_eq!(d, 961.8288);
    }

    #[test]
    fn distance_array() {
        let point_0 = waypoint(-73.9761399, 40.7767644);
        let point_1 = waypoint(-73.9673991, 40.771209);
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

        let res = fit_bounds(gpx_ref.clone(), &[]);
        assert_eq!(None, res.metadata.and_then(|m| m.bounds))
    }

    #[test]
    fn find_bounding_box() {
        let points = vec![
            waypoint(-73.9761399, 40.7767644),
            waypoint(-73.9673991, 40.771209),
        ];
        let rect = find_bounds(&points).unwrap();
        assert_eq!(40.771209, rect.min().x);
        assert_eq!(-73.9761399, rect.min().y);
        assert_eq!(40.7767644, rect.max().x);
        assert_eq!(-73.9673991, rect.max().y);
    }

    #[test]
    //test for a point that is within the boundaries defined by start and end
    fn interception_inside() {
        let p = waypoint(52.5186118, 13.408056);
        let ip = intercept(
            &p,
            (&waypoint(64.15, -21.933333), &waypoint(55.75, 37.616667)),
        );
        assert_approx_eq!(61.6561898, ip.point().x());
        assert_approx_eq!(20.543903, ip.point().y());
    }

    #[test]
    //test for a point that is outside the boundaries defined by start and end
    fn interception_outside() {
        let p = waypoint(4.0, 2.0);
        let ip = intercept(&p, (&waypoint(-1.0, 0.0), &waypoint(1.0, 0.0)));
        assert_approx_eq!(4.0, ip.point().x(), 1.0e-3);
        assert_approx_eq!(0.0, ip.point().y());
    }

    #[test]
    fn distance_to_line() {
        //0.00028° = 0°0'1" ~ 30.9 m
        let p = waypoint(0.0, 0.00028);
        let ip = intercept(&p, (&waypoint(-1.0, 0.0), &waypoint(1.0, 0.0)));
        let dist_p_ip = distance(&p, &ip);
        assert_approx_eq!(30.9607975, dist_p_ip);
    }

    #[test]
    fn near_segment() {
        let segment = (&waypoint(0.0, 0.5), &waypoint(0.0, 1.0));
        assert!(is_near_segment(&waypoint(0.0, 0.5), segment, 0.1));
        assert!(is_near_segment(&waypoint(0.0, 1.0), segment, 0.1));
        assert!(is_near_segment(&waypoint(0.0, 1.00001), segment, 2.3));
        assert!(!is_near_segment(&waypoint(0.0, 1.5), segment, 0.1));
    }
}
