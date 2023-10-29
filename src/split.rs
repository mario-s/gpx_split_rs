use std::fmt::Debug;
use std::io::{Error, ErrorKind};

use gpx::{Gpx, Track, TrackSegment};

use crate::dist;
use crate::io;

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
        let mut counter: u32 = 1;
        let gpx = io::read_gpx(self.file).unwrap();

        let mut track_segment: TrackSegment = TrackSegment::new();

        for track in &gpx.tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                track_segment.points.push(point.clone());

                    if self.strategy.exceeds_limit(track_segment.to_owned()) {
                        self.write_gpx(&gpx, &track, &track_segment, counter).unwrap();

                        counter += 1;
                        //we start a fresh with a clear vector of points
                        track_segment.points.clear();
                        //add current point as first one to new segment
                        track_segment.points.push(point);
                    }
            });
        }

        //this will be in most cases true
        //but it can happen that we split at the end
        if track_segment.points.len() > 1 {
            match gpx.tracks.last() {
                Some(last) => self.write_gpx(&gpx, last, &track_segment, counter).unwrap(),
                None => ()
            }
        }
    }

    fn write_gpx(&self, src_gpx: &Gpx, src_track: &Track, segment: &TrackSegment, counter: u32) -> Result<(), Error> {
        let mut gpx = src_gpx.clone();
        gpx.tracks.clear();

        let mut track: Track = src_track.clone();
        track.segments.clear();
        track.segments.push(segment.to_owned());

        gpx.tracks.push(track);

        let path = self.create_path(counter)?;
        io::write_gpx(gpx, path)
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
    max_points: u32,
}

impl PointsSplitter {
    pub fn new(max_points: u32) -> Self {
        PointsSplitter { max_points: max_points }
    }
}

impl Splitter for PointsSplitter {
    fn exceeds_limit(&self, track_segment: TrackSegment) -> bool {
        track_segment.points.len() > self.max_points.try_into().unwrap()
    }
}

/// -------------------------------------------------

/// strategy to split based on the lenth of a segment
///
#[derive(Debug)]
pub struct LengthSplitter {
    max_length: f64,
}

impl LengthSplitter {
    pub fn new(max_length: f64) -> Self {
        LengthSplitter { max_length: max_length }
    }
}

impl Splitter for LengthSplitter {
    fn exceeds_limit(&self, track_segment: TrackSegment) -> bool {
        dist::distance_track(track_segment) > self.max_length
    }
}
