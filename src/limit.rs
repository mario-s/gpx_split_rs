use gpx::Waypoint;
use log::debug;

use crate::geo::distance_points;

/// creates a closure that returns true when the amount of points
/// is equal or greater than max
///
pub fn points(max: u32) -> Box<dyn Fn(&[Waypoint]) -> bool> {
    debug!("maximum number of points: {}", max);
    Box::new(move |points| points.len() >= max as usize)
}

/// creates a closure that returns true when the sum of the distance between all points
/// is greater tham max
///
pub fn length(max: u32) -> Box<dyn Fn(&[Waypoint]) -> bool> {
    debug!("maximum length between points: {}", max);
    Box::new(move |points| distance_points(points) > max.into())
}