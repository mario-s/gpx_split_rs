extern crate gpx;
use std::fmt::Debug;
use std::io::BufReader;
use std::fs::File;

use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

/// Calculates the distance between multiple points.
/// Returns result in Meter.
///
fn distance(points: Vec<Point>) -> f64 {
    distance_vec(points, Unit::Meters)
}

#[test]
fn test_distance() {
    let point_0 = Point::new(40.7767644, -73.9761399);
    let point_1 = Point::new(40.771209, -73.9673991);
    let distance = distance(vec![point_0, point_1]);
    assert!(distance == 960.9072987659282);
}

/// Calculates the distance of all points in the track.
/// Returns result in Meter.
///
fn distance_track(track_segment: TrackSegment) -> f64 {
    let points = track_segment.points.iter().map(|p| p.point()).map(|p| Point::new(p.x(), p.y())).collect();
    return distance(points);
}

/// -------------------------------------------------

pub struct Context<'a, S> {
    pub file: &'a str,
    pub strategy: S,
}

impl<S> Context<'_, S>
where
    S: Splitter + Debug,
{
    fn read_tracks(&self) -> Vec<Track> {
        let file = File::open(self.file).unwrap();
        let reader = BufReader::new(file);

        //gives a Result<Gpx, Error>
        let gpx: Gpx = read(reader).unwrap();

        return gpx.tracks;
    }

    pub fn execute(&mut self) {
        println!("Common preamble");
        let mut counter: u32 = 1;
        let tracks = self.read_tracks();
        let mut track_segment: TrackSegment = TrackSegment::new();

        for track in tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                track_segment.points.push(point);

                    if self.strategy.exceeds_limit(track_segment.to_owned()) {
                        //TODO: if a limit for the track segment is exceeded, we write current segment to a file and create a new one
                        println!("Splitting {} with {:?}", counter, self.strategy);
                        counter += 1;
                    }
            });
        }


        println!("Common postamble");
    }
}

/// -------------------------------------------------

pub trait Splitter {
    fn exceeds_limit(&self, track_segment: TrackSegment) -> bool;
}

/// -------------------------------------------------

/// strategy to split based on the number of points
///
#[derive(Debug)]
pub struct PointsSplitter {
    max_limit: u32,
}

impl PointsSplitter {
    pub fn new(max_limit: u32) -> Self {
        PointsSplitter { max_limit }
    }
}

impl Splitter for PointsSplitter {

    fn exceeds_limit(&self, track_segment: TrackSegment) -> bool {
        track_segment.points.len() > self.max_limit.try_into().unwrap()
    }
}

/// -------------------------------------------------

/// strategy to split based on the lenth of a segment
///
#[derive(Debug)]
pub struct LengthSplitter {
    max_limit: f64,
}

impl LengthSplitter {
    pub fn new(max_limit: f64) -> Self {
        LengthSplitter { max_limit }
    }
}

impl Splitter for LengthSplitter {
    fn exceeds_limit(&self, track_segment: TrackSegment) -> bool {
        distance_track(track_segment) > self.max_limit
    }
}
