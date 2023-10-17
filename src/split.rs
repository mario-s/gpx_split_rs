
extern crate gpx;
use std::default::Default;
use std::io::BufReader;
use std::fs::File;

use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment, Waypoint};

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point>) -> f64 {
    distance_vec(points, Unit::Meters)
}

#[test]
fn test_distance_vec() {
    let point_0 = Point::new(40.7767644, -73.9761399);
    let point_1 = Point::new(40.771209, -73.9673991);
    let distance = distance(vec![point_0, point_1]);
    assert!(distance == 960.9072987659282);
}

/// -------------------------------------------------

pub struct Context<S> {
    pub strategy: S,
    output_count: u32
}

impl<S> Default for Context<S>
where
    S: Default + Splitter,
{
    fn default() -> Self {
        Context {
            strategy: Default::default(),
            output_count: 1,
        }
    }
}

impl<S> Context<S>
where
    S: Splitter,
{
    pub fn execute(&mut self) {
        println!("Common preamble");
        let mut trkseg: TrackSegment = TrackSegment::new();

        self.strategy.execute();

        println!("Common postamble");
        println!("Limit: {}", self.strategy.max_limit());
        self.output_count = 1;
    }
}

/// -------------------------------------------------

pub trait Splitter {
    fn execute(&self);
    fn max_limit(&self) -> u32;
}

/// -------------------------------------------------

/// strategy to split based on the number of points
///
#[derive(Default)]
pub struct PointsSplitter {
    max_limit: u32,
}

impl PointsSplitter {
    pub fn new(max_limit: u32) -> Self {
        PointsSplitter { max_limit }
    }
}

impl Splitter for PointsSplitter {
    fn execute(&self) {
        println!("PointsSplitter")
    }

    fn max_limit(&self) -> u32 {
        self.max_limit
    }
}

/// -------------------------------------------------

/// strategy to split based on the lenth of a segment
///
#[derive(Default)]
pub struct LengthSplitter {
    max_limit: u32,
}

impl LengthSplitter {
    pub fn new(max_limit: u32) -> Self {
        LengthSplitter { max_limit }
    }
}

impl Splitter for LengthSplitter {

    fn execute(&self) {
        println!("LengthSplitter")
    }

    fn max_limit(&self) -> u32 {
        self.max_limit
    }
}
