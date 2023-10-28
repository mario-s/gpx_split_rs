extern crate gpx;
use std::fmt::Debug;
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;

use gpx::errors::GpxError;
use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use haversine_rs::distance_vec;

use gpx::read;
use gpx::write;
use gpx::{Gpx, GpxVersion, Track, TrackSegment};

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
    distance(points)
}

/// Function to convert an GpxError
fn to_error(gpx_err: GpxError) -> Error {
    Error::new(ErrorKind::Other, gpx_err.to_string())
}

/// -------------------------------------------------

pub struct Context<'a, S> {
    pub file: &'a str,
    pub strategy: S,
}

impl<S> Context<'_, S>
where
    S: Splitter,
{
    pub fn new(file: &'static str, strategy: S) -> Self {
        Context { file, strategy }
    }

    pub fn execute(&mut self) {
        println!("Common preamble");
        let mut counter: u32 = 1;
        let gpx = self.read_gpx().unwrap();

        let mut track_segment: TrackSegment = TrackSegment::new();

        for track in gpx.tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                track_segment.points.push(point);

                    if self.strategy.exceeds_limit(track_segment.to_owned()) {
                        self.write_gpx(&track, &track_segment, counter).unwrap();

                        counter += 1;
                        track_segment = TrackSegment::new();
                    }
            });
        }
        //TODO write remaining

        println!("Common postamble");
    }

    fn read_gpx(&self) -> Result<Gpx, Error> {
        let file = File::open(self.file)?;
        let reader = BufReader::new(file);

        match read(reader) {
            Ok(gpx) => Ok(gpx),
            Err(gpx_err) => Err(to_error(gpx_err))
        }
    }

    fn write_gpx(&self, src_track: &Track, segment: &TrackSegment, counter: u32) -> Result<(), Error> {
        let mut gpx : Gpx = Default::default();
        gpx.version = GpxVersion::Gpx11;

        let mut track: Track = src_track.clone();
        track.segments.clear();
        track.segments.push(segment.to_owned());
        gpx.tracks.push(track);

        let path = self.create_path(counter)?;
        let file = File::create(path)?;
        let res = write(&gpx, file);
        match res {
            Ok(_) => Ok(()),
            Err(gpx_err) => Err(to_error(gpx_err))
        }
    }

    fn create_path(&self, counter: u32) -> Result<String, Error> {
        let parts: Vec<&str> = self.file.rsplitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidInput, format!("invalid file: {}", self.file)));
        }
        let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
        Ok(name)
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
