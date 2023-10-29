use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::fs::File;

extern crate gpx;
use gpx::write;
use gpx::{Gpx, GpxVersion, Track, TrackSegment};

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
        println!("Common preamble");
        let mut counter: u32 = 1;
        let gpx = io::read_gpx(self.file).unwrap();

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
            Err(gpx_err) => Err(io::to_error(gpx_err))
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