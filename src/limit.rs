use gpx::Waypoint;
use log::debug;
use log::trace;

use crate::io::read_gpx;
use crate::geo::distance_all;
use crate::geo::interception_points;


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
    /// Creates a new limit of points.
    pub fn points(max_points: u32) -> Self {
        debug!("maximum number of points: {}", max_points);
        Limit::Points(max_points)
    }

    /// Creates a new limit of length in meter.
    pub fn length(max_length: u32) -> Self {
        debug!("maximum length between points: {}", max_length);
        Limit::Length(max_length)
    }

    /// Create a bnew limit for a distance to locations.
    pub fn location(waypoint_file: String, distance: u32) -> Self {
        trace!("reading waypoints for splitting at location from: {}", waypoint_file);
        //there is nothing much we can do here, just give up with a helpful error message
        let gpx = read_gpx(&waypoint_file).expect("can't read file with splitting points");
        debug!("minimum distance for location to split: {}", distance);
        let waypoints = gpx.waypoints;
        debug!("number of waypoints for splitting: {}", waypoints.len());
        Limit::Location(Box::new(waypoints), distance)
    }

    pub fn exceeds(&self, points: &mut [Waypoint]) -> bool {
        match self {
            Limit::Points(max_points) => points.len() >= *max_points as usize,
            Limit::Length(max_length) => distance_all(points) > *max_length as f64,
            Limit::Location(split_points, dist) => self.exceeds_loc(split_points, *dist, points),
        }
    }

    fn exceeds_loc(&self, split_points: &[Waypoint], dist: u32, points: &mut [Waypoint]) -> bool {
        let len = points.len();
        if split_points.is_empty() || len < 2 {
            return false;
        }
        let line = (&points[len - 2], &points[len - 1]);
        let mut map = interception_points(dist, split_points, line);
        //replace last point with the interception point, that has the shortest distance
        match map.pop_first() {
            Some(pair) => {
                debug!("shortest distance in milimeter: {}", pair.0);
                points[len-1] = pair.1;
                true
            },
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::Point;
    use gpx::Waypoint;

    use crate::limit::Limit;

    #[test]
    fn location() {
        let lim = Limit::location("target/debug/pois.gpx".to_string(), 10);
        match lim {
            Limit::Location(waypoints, dist) => {
                assert_eq!(10, waypoints.len());
                assert_eq!(10, dist)
            },
            _ => panic!("unexpected result")
        }
    }

    #[test]
    #[should_panic(expected = "can't read file with splitting points: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }")]
    fn wrong_location() {
        Limit::location("pois.gpx".to_string(), 10);
    }

    #[test]
    fn exceeds_location_false() {
        let lim = Limit::Location(Box::default(), 2);
        assert!(!lim.exceeds(&mut [Waypoint::default()]));
        let lim = Limit::Location(Box::new(vec![Waypoint::default()]), 2);
        assert!(!lim.exceeds(&mut [Waypoint::default()]));
    }

    #[test]
    fn exceeds_location_true() {
        let lim = Limit::Location(Box::new(vec![Waypoint::new(Point::new(13.535369, 52.643826)), Waypoint::new(Point::new(13.535368, 52.643825))]), 15);
        let points = &mut [Waypoint::new(Point::new(13.533826, 52.643605)), Waypoint::new(Point::new(13.535629, 52.644021))];
        assert!(lim.exceeds(points));
    }
}