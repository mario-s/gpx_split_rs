use std::fmt::Debug;
use gpx::Waypoint;

use crate::geo;

/// checks if the points exceed a defined limit.
pub trait Limit {
    fn exceeds_limit(&self, points: &Vec<Waypoint>) -> bool;
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
        PointsLimit { max_points }
    }
}

impl Limit for PointsLimit {
    fn exceeds_limit(&self, points: &Vec<Waypoint>) -> bool {
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
        LengthLimit { max_length }
    }
}

impl Limit for LengthLimit {
    fn exceeds_limit(&self, points: &Vec<Waypoint>) -> bool {
        geo::distance_points(&points) > self.max_length.into()
    }
}
