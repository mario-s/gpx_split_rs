use std::collections::HashMap;
use gpx::Waypoint;
use log::debug;

use crate::geo::distance;
use crate::geo::distance_all;
use crate::geo::interception_point;


/// checks if the points exceed a defined limit.
pub enum Limit {
    /// strategy to check limit based on the number of points
    Points(u32),
    /// strategy to check limit based on the length in meter of the sum of the distances between the points
    Length(u32),
    /// strategy to check limit based on the distance in meter to the nearest location
    Location(Box<Vec<Waypoint>>, u32),
}

impl Limit {
    pub fn points(max_points: u32) -> Self {
        debug!("maximum number of points: {}", max_points);
        Limit::Points(max_points)
    }

    pub fn length(max_length: u32) -> Self {
        debug!("maximum length between points: {}", max_length);
        Limit::Length(max_length)
    }

    pub fn location(waypoint_file: String, distance: u32) -> Self {
        debug!("reading waypoints for splitting at location from: {}", waypoint_file);
        debug!("minimum distance for location to split: {}", distance);
        let waypoints: Vec<Waypoint> = vec![];
        Limit::Location(Box::new(waypoints), distance)
    }

    pub fn exceeds(&self, points: &[Waypoint]) -> bool {
        match self {
            Limit::Points(max_points) => points.len() >= *max_points as usize,
            Limit::Length(max_length) => distance_all(points) > *max_length as f64,
            Limit::Location(split_points, dist) => self.exceeds_loc(*dist, *&split_points, points),
        }
    }

    fn exceeds_loc(&self, dist: u32, split_points: &Box<Vec<Waypoint>>, points: &[Waypoint]) -> bool {
        let len = points.len();
        if split_points.is_empty() || len < 2 {
            return false;
        }
        let dist = dist as f64;
        //map of distances and interception points
        let map = split_points.iter().filter_map(|p| {
            let line = (&points[len - 2], &points[len - 1]);
            let ip = interception_point(&p, line);
            let d = distance(&p, &ip);
            if d < dist {
                let d = (d * 1000.0) as i64;
                Some((d, ip))
            } else {
                None
            }
        }).collect::<HashMap<_, _>>();
        //TODO: replace last point with the interception point, that has the shortest distance

        return !map.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use geo_types::Point;
    use gpx::Waypoint;

    use crate::limit::Limit;

    #[test]
    fn location() {
        let lim = Limit::location("waypoint_file".to_string(), 10);
        match lim {
            Limit::Location(waypoints, dist) => {
                assert_eq!(0, waypoints.len());
                assert_eq!(10, dist)
            },
            _ => panic!("unexpected result")
        }
    }

    #[test]
    fn exceeds_location_false() {
        let lim = Limit::Location(Box::new(vec![]), 2);
        assert!(!lim.exceeds(&[Waypoint::default()]));
        let lim = Limit::Location(Box::new(vec![Waypoint::default()]), 2);
        assert!(!lim.exceeds(&[Waypoint::default()]));
    }

    #[test]
    fn exceeds_location_true() {
        let lim = Limit::Location(Box::new(vec![Waypoint::new(Point::new(13.535369, 52.643826))]), 15);
        let points = &[Waypoint::new(Point::new(13.533826, 52.643605)), Waypoint::new(Point::new(13.535629, 52.644021))];
        assert!(lim.exceeds(points));
    }
}