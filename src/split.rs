use std::fmt::Debug;
use std::io::{Error, ErrorKind};

use gpx::{Gpx, Track, TrackSegment, Waypoint};

use crate::dist;
use crate::io;

pub struct Context<S> {
    pub path: String,
    pub strategy: S,
}

impl<S> Context<S>
where
    S: Splitter,
{
    pub fn new(path: String, strategy: S) -> Self {
        Context { path, strategy }
    }

    pub fn execute(&mut self) -> Result<u32, Error> {
        let gpx = io::read_gpx(self.path.as_str())?;

        let mut counter: u32 = 1;
        let mut points = Vec::new();
        let tracks = &gpx.tracks;

        for track in tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                points.push(point.clone());

                if self.strategy.exceeds_limit(&points) {
                    if let Err(_) = self.write_track(&gpx, track, &points, counter) {
                        return;
                    }

                    counter += 1;
                    //we start a fresh with a clear vector of points
                    points.clear();
                    //add current point as first one to new segment
                    points.push(point);
                }
            });
        }

        //this will be true in most cases
        //but it can happen that we split at the end
        if points.len() > 1 {
            if let Some(last) = gpx.tracks.last() {
                self.write_track(&gpx, last, &points, counter)?
            }
        }
        Ok(counter)
    }

    fn write_track(&self, src_gpx: &Gpx, src_track: &Track, points: &Vec<Waypoint>, counter: u32) -> Result<(), Error> {
        let mut gpx = src_gpx.clone();
        gpx.tracks.clear();

        let mut track_segment = TrackSegment::new();
        track_segment.points.append(&mut points.to_owned());

        let mut track: Track = src_track.clone();
        track.segments.clear();
        track.segments.push(track_segment);

        gpx.tracks.push(track);

        let path = self.create_path(counter)?;
        io::write_gpx(gpx, path)
    }

    fn create_path(&self, counter: u32) -> Result<String, Error> {
        let parts: Vec<&str> = self.path.rsplitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidInput, format!("invalid file: {}", self.path)));
        }
        let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
        Ok(name)
    }
}

/// -------------------------------------------------

pub trait Splitter {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool;
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
        PointsSplitter { max_points }
    }
}

impl Splitter for PointsSplitter {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool {
        points.len() > self.max_points.try_into().unwrap()
    }
}

/// -------------------------------------------------

/// strategy to split based on the lenth of a segment
///
#[derive(Debug)]
pub struct LengthSplitter {
    max_length: u32,
}

impl LengthSplitter {
    pub fn new(max_length: u32) -> Self {
        LengthSplitter { max_length }
    }
}

impl Splitter for LengthSplitter {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool {
        dist::distance_points(points.to_owned()) > self.max_length.into()
    }
}
