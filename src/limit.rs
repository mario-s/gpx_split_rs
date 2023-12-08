use std::fmt::Debug;
use gpx::Waypoint;
use log::debug;

use crate::geo::distance_points;

/// checks if the points exceed a defined limit.
pub trait Limit {
    fn exceeds(&self, points: &[Waypoint]) -> bool;
}

//-------------------------------------------------

/// strategy to check limit based on the number of points
///
#[derive(Debug, Clone)]
pub struct PointsLimit {
    max_points: u32,
}

impl PointsLimit {
    pub fn new(max_points: u32) -> Self {
        debug!("maximum number of points: {}", max_points);
        PointsLimit { max_points }
    }
}

impl Limit for PointsLimit {
    fn exceeds(&self, points: &[Waypoint]) -> bool {
        points.len() >= self.max_points.try_into().unwrap()
    }
}

//-------------------------------------------------

/// strategy to check limit based on the length of the sum of the distances between the points
///
#[derive(Debug, Clone)]
pub struct LengthLimit {
    max_length: u32,
}

impl LengthLimit {
    pub fn new(max_length: u32) -> Self {
        debug!("maximum length between points: {}", max_length);
        LengthLimit { max_length }
    }
}

impl Limit for LengthLimit {
    fn exceeds(&self, points: &[Waypoint]) -> bool {
        distance_points(points) > self.max_length.into()
    }
}
