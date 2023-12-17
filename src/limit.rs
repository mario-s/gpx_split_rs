use gpx::Waypoint;
use log::debug;

use crate::geo::distance_points;


/// checks if the points exceed a defined limit.
pub enum Limit {
    /// strategy to check limit based on the number of points
    Points(u32),
    /// strategy to check limit based on the length of the sum of the distances between the points
    Length(u32),
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

    pub fn exceeds(&self, points: &[Waypoint]) -> bool {
        match self {
            Limit::Points(max_points) => points.len() >= *max_points as usize,
            Limit::Length(max_length) => distance_points(points) > *max_length as f64,
        }
    }
}
