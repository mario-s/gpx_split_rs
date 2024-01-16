use std::collections::BTreeMap;
use gpx::Waypoint;
use log::debug;
use log::trace;

use crate::io::read_gpx;
use crate::geo::{distance, distance_all, interception_point, is_near_segment};


/// Checks if the points exceed a defined limit.
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

    /// Creates a new limit for a distance to locations.
    pub fn location(waypoint_file: String, distance: u32) -> Self {
        trace!("reading waypoints for splitting at location from: {}", waypoint_file);
        //there is nothing much we can do here, just give up with a helpful error message
        let gpx = read_gpx(&waypoint_file).expect("can't read file with splitting points");
        debug!("minimum distance for location to split: {}", distance);
        let waypoints = gpx.waypoints;
        debug!("number of waypoints for splitting: {}", waypoints.len());
        Limit::Location(Box::new(waypoints), distance)
    }

    /// If the points exceed a defined limit, (see enum values of [Limit])
    /// this method returns true, else false.
    pub fn exceeds(&mut self, points: &mut [Waypoint]) -> bool {
        match self {
            Limit::Points(max_points) => points.len() >= *max_points as usize,
            Limit::Length(max_length) => distance_all(points) > *max_length as f64,
            Limit::Location(ref mut split_points, dist) => Limit::exceeds_location(*dist, split_points, points),
        }
    }

    fn exceeds_location(dist: u32, split_points: &mut Vec<Waypoint>, points: &mut [Waypoint]) -> bool {
        let len = points.len();
        // early return when there are no splitting points or we don't have a line yet
        if split_points.is_empty() || len < 2 {
            return false;
        }

        let segment = (&points[len - 2], &points[len - 1]);
        let mut map = Limit::interception_points(dist, split_points, segment);

        //replace last point with the interception point, that has the shortest distance
        match map.pop_first() {
            Some(entry) => {
                debug!("shortest distance in milimeter: {}", entry.0);
                //Each split point can be used only once. When we add an interception point,
                //we will also remove the splitting point. The consequences of keeping the splitting point
                //would be weird tracks, containing just a few points till the next interception point.
                let index = entry.1.0;
                split_points.remove(index);
                //finally the interception point
                let point = entry.1.1;
                points[len-1] = point;
                true
            },
            None => false
        }
    }

    // This creates a map of distances and interception points from each split point to the segment.
    // If the distance is above the min_dist, the interception point is not considered.
    // The map is sorted, where the first entry is the shortest distance with the corresponding interception point.
    // The unit of the distance is milimeter.
    fn interception_points(dist: u32, split_points: &mut [Waypoint], segment: (&Waypoint, &Waypoint)) -> BTreeMap<i64, (usize, Waypoint)> {
        let min_dist = dist as f64;

        split_points.iter().enumerate().filter_map(|(index, split_point)| {
            let mut ip = interception_point(split_point, segment);
            // The interception point can be far off from the segment.
            // So we consider only those which are within the distance.
            if !is_near_segment(&ip, segment, dist as f64) {
                return None;
            }
            let dist = distance(split_point, &ip);
            if dist < min_dist {
                debug!("interception point is near segment with a distance of {} meter", dist);
                let dist = (dist * 1000.0) as i64;
                if let Some(name) = &split_point.name {
                    ip.name = Some(format!("nearby {}", name).to_string());
                }
                Some((dist, (index, ip)))
            } else {
                None
            }
        }).collect::<BTreeMap<_, _>>()
    }
}


#[cfg(test)]
mod tests {
    use geo_types::Point;
    use gpx::Waypoint;

    use super::*;

    fn waypoint(x: f64, y: f64) -> Waypoint {
        Waypoint::new(Point::new(x, y))
    }

    #[test]
    fn location() {
        let lim = Limit::location("target/debug/pois.gpx".to_string(), 10);
        match lim {
            Limit::Location(waypoints, dist) => {
                assert_eq!(2, waypoints.len());
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
        let mut lim = Limit::Location(Box::default(), 2);
        assert!(!lim.exceeds(&mut [Waypoint::default()]));
        let mut lim = Limit::Location(Box::new(vec![Waypoint::default()]), 2);
        assert!(!lim.exceeds(&mut [Waypoint::default()]));
    }

    #[test]
    fn exceeds_location_true() {
        let split_points = Box::new(vec![waypoint(13.535369, 52.643826), waypoint(13.535368, 52.643825)]);
        let mut lim = Limit::Location(split_points, 15);
        let points = &mut [waypoint(13.533826, 52.643605), waypoint(13.535629, 52.644021)];
        assert!(lim.exceeds(points));
    }

    #[test]
    fn interception_points_not_near() {
        let dist = 34000;
        let line = (&waypoint(-1.0, 0.0), &waypoint(1.0, 0.0));
        let mut split_points = Box::new(vec![waypoint(-1.5, 1.5)]);
        let ips = Limit::interception_points(dist, &mut split_points, line);
        assert!(ips.is_empty());
    }

    #[test]
    fn interception_points_near() {
        let dist = 34000;
        let line = (&waypoint(-1.0, 0.0), &waypoint(1.0, 0.0));
        let mut wp = waypoint(0.0, 0.2);
        wp.name = Some("Point".to_string());
        let mut split_points = Box::new(vec![waypoint(-0.5, 1.5), waypoint(-0.1, 0.4),
            wp, waypoint(0.5, 0.3)]);

        let mut ips = Limit::interception_points(dist, &mut split_points, line);

        assert_eq!(2, ips.len());
        let first = ips.pop_first();
        let first = first.unwrap_or((0, (0, Waypoint::default())));
        let point = first.1.1;
        let first = first.0;
        let second = ips.pop_first();
        let second = second.unwrap_or((0, (0, Waypoint::default()))).0;
        let dist = (dist * 1000) as i64; //convert to milimeter
        assert!(second < dist);
        assert!(first < second);
        assert_eq!("nearby Point", point.name.unwrap());
    }
}