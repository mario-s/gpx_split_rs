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
    S: Limit,
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
                    //we start afresh with a clear vector of points
                    points.clear();
                    //add current point as first one to new segment
                    points.push(point);
                }
            });
        }

        //this condition will be true in most cases
        //but it can happen that we split at the end of track, in this case we have only one point
        if points.len() > 1 {
            if let Some(last) = gpx.tracks.last() {
                self.write_track(&gpx, last, &points, counter)?
            }
        }
        Ok(counter)
    }

    fn spilt_tracks(&self, tracks: &Vec<Track>) -> Vec<Track> {
        let mut new_tracks = Vec::new();
        let mut points = Vec::new();
        for track in tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                points.push(point.clone());
                if self.strategy.exceeds_limit(&points) {

                    let new_track = self.new_track(track, &points);
                    new_tracks.push(new_track);

                    points.clear();
                }
            });
        }
        //this condition will be true in most cases
        //but it can happen that we split at the end of track, in this case we have only one point
        if points.len() > 1 {
            if let Some(last) = tracks.last() {
                let new_track = self.new_track(last, &points);
                new_tracks.push(new_track);
            }
        }
        new_tracks
    }

    fn new_track(&self, src_track: &Track, points: &Vec<Waypoint>) -> Track {
        let mut clone_track = src_track.clone();

        let mut track_segment = TrackSegment::new();
        track_segment.points.append(&mut points.to_owned());
        clone_track.segments.clear();
        clone_track.segments.push(track_segment);

        clone_track
    }

    fn write_track(&self, src_gpx: &Gpx, src_track: &Track, points: &Vec<Waypoint>, counter: u32) -> Result<(), Error> {
        //clone the source gpx and just clear the tracks to keep the rest
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
        //new file name would be like foo_1.gpx
        let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
        Ok(name)
    }
}

//-------------------------------------------------

/// Checks if the points exceed a defined limit.
pub trait Limit {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool;
}

//-------------------------------------------------

/// strategy to check limit based on the number of points
///
#[derive(Debug)]
pub struct PointsLimit {
    max_points: u32,
}

impl PointsLimit {
    pub fn new(max_points: u32) -> Self {
        PointsLimit { max_points }
    }
}

impl Limit for PointsLimit {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool {
        points.len() > self.max_points.try_into().unwrap()
    }
}

//-------------------------------------------------

/// strategy to check limit based on the length of the sum of the distances between the points
///
#[derive(Debug)]
pub struct LengthLimit {
    max_length: u32,
}

impl LengthLimit {
    pub fn new(max_length: u32) -> Self {
        LengthLimit { max_length }
    }
}

impl Limit for LengthLimit {
    fn exceeds_limit(&self, points: &[Waypoint]) -> bool {
        dist::distance_points(points.to_owned()) > self.max_length.into()
    }
}
