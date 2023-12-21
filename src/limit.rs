use gpx::Waypoint;
use log::debug;

use crate::geo::distance_points;


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
            Limit::Length(max_length) => distance_points(points) > *max_length as f64,
            Limit::Location(_, _) => panic!("not implemented yet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::limit::Limit;

    #[test]
    fn test_location() {
        let lim = Limit::location("waypoint_file".to_string(), 10);
        match lim {
            Limit::Location(waypoints, dist) => {
                assert_eq!(0, waypoints.len());
                assert_eq!(10, dist)
            },
            _ => panic!("unexpected result")
        }
    }
}