use std::io::Error;
use gpx::{Gpx, Track, TrackSegment, Waypoint, Route};

use crate::limit::Limit;
use crate::io;


pub struct RouteSplitter <L> {
    path: String,
    limit: L,
}

impl<L> RouteSplitter<L> where L: Limit {

    pub fn new(path: String, limit: L) -> Self {
        RouteSplitter { path, limit }
    }

    pub fn split(&self) -> Result<usize, Error> {
        let gpx = io::read_gpx(self.path.as_str())?;
        self.spilt_routes(&gpx.routes);
        Ok(0)
    }

    fn spilt_routes(&self, routes: &Vec<Route>) -> Vec<Route> {
        let mut new_routes = vec![];
        let mut points = vec![];
        for route in routes {
            route.points.iter().for_each(|point| {
                points.push(point.clone());

                if self.limit.exceeds_limit(&points) {
                    let new_route = self.clone_route(route, &mut points);
                    new_routes.push(new_route);

                    points.clear();
                    //add current point as first one to new segment
                    points.push(point.to_owned());
                }
            });
        }

        new_routes
    }

    fn clone_route(&self, src_route: &Route, points: &mut Vec<Waypoint>) -> Route {
        let mut cloned_route = src_route.clone();
        cloned_route.points.clear();
        cloned_route.points.append(points);
        cloned_route
    }
}

//--------------------------------------------------------------
pub struct TrackSplitter<L> {
    path: String,
    limit: L,
}

impl<L> TrackSplitter<L> where L: Limit {

    pub fn new(path: String, limit: L) -> Self {
        TrackSplitter { path, limit }
    }

    pub fn split(&self) -> Result<usize, Error> {
        let gpx = io::read_gpx(self.path.as_str())?;
        let tracks = self.spilt_tracks(&gpx.tracks);
        self.write_tracks(gpx, tracks)
    }

    /// splits the given tracks into new tracks where the number of points of that tracks are limted
    ///
    fn spilt_tracks(&self, tracks: &Vec<Track>) -> Vec<Track> {
        let mut new_tracks = vec![];
        let mut points = vec![];
        for track in tracks {
            track.segments.iter()
            .flat_map(|segment| segment.points.iter().cloned())
            .for_each(|point| {
                points.push(point.clone());

                //create a new track when the points exceed a limit
                if self.limit.exceeds_limit(&points) {
                    let new_track = self.clone_track(track, &points);
                    new_tracks.push(new_track);

                    points.clear();
                    //add current point as first one to new segment
                    points.push(point);
                }
            });
        }
        //this condition will be true in most cases
        //but it can happen that we split at the end of track, in this case we have only one point
        if points.len() > 1 {
            if let Some(last) = tracks.last() {
                let new_track = self.clone_track(last, &points);
                new_tracks.push(new_track);
            }
        }
        new_tracks
    }

    /// clone the source track and add new track segment with the points
    ///
    fn clone_track(&self, src_track: &Track, points: &Vec<Waypoint>) -> Track {
        let mut track_segment = TrackSegment::new();
        track_segment.points.append(&mut points.to_owned());

        let mut cloned_track = src_track.clone();
        cloned_track.segments.clear();
        cloned_track.segments.push(track_segment);

        cloned_track
    }

    /// writes the given tracks into new files
    ///
    fn write_tracks(&self, src_gpx: Gpx, tracks: Vec<Track>) -> Result<usize, Error> {
        let len = tracks.len();

        for (index, track) in tracks.iter().enumerate() {
            self.write_track(&src_gpx, track, index)?;
        }

        Ok(len)
    }

    /// writes a single track into a file, counter is the suffix for the file name
    ///
    fn write_track(&self, src_gpx: &Gpx, track: &Track, counter: usize) -> Result<(), Error> {
        //clone the source gpx and just clear the tracks to keep the rest
        let mut gpx = src_gpx.clone();
        gpx.tracks.clear();
        gpx.tracks.push(track.to_owned());

        let path = io::create_path(&self.path, counter)?;
        io::write_gpx(gpx, path)
    }
}

//--------------------------------------------------------------

#[test]
fn test_split_track_zero() {
    let track = Track::new();

    let lim = crate::limit::PointsLimit::new(0);
    let split = TrackSplitter::new("".to_string(), lim);
    let tracks = split.spilt_tracks(&vec![track]);
    assert_eq!(0, tracks.len());
}

#[test]
fn test_split_track_2() {
    let mut segment = TrackSegment::new();
    for i in 0..4 {
        let mut point = Waypoint::default();
        point.name = Some(format!("point {}", i));
        segment.points.push(point);
    }
    let mut track = Track::new();
    track.segments.push(segment);
    let lim = crate::limit::PointsLimit::new(2);
    let split = TrackSplitter::new("".to_string(), lim);
    let tracks = split.spilt_tracks(&vec![track]);

    //expect 2 tracks with 1 segment each containing 2 points
    assert_eq!(3, tracks.len());

    let first_points = tracks.first()
        .and_then(|t| t.segments.first())
        .and_then(|s| Some(s.points.clone())).unwrap();
    let middle_points = tracks.get(1)
        .and_then(|t| t.segments.first())
        .and_then(|s| Some(s.points.clone())).unwrap();
    let last_points = tracks.last()
        .and_then(|t| t.segments.first())
        .and_then(|s| Some(s.points.clone())).unwrap();
    assert_eq!(2, first_points.len());
    assert_eq!(2, middle_points.len());
    assert_eq!(2, last_points.len());

    //first track from 0 to 1
    assert_eq!("point 0", first_points.first().and_then(|p| p.name.clone()).unwrap());
    assert_eq!("point 1", first_points.last().and_then(|p| p.name.clone()).unwrap());
    //second track from 1 to 2
    assert_eq!("point 1", middle_points.first().and_then(|p| p.name.clone()).unwrap());
    assert_eq!("point 2", middle_points.last().and_then(|p| p.name.clone()).unwrap());
    //third track from 2 to 3
    assert_eq!("point 2", last_points.first().and_then(|p| p.name.clone()).unwrap());
    assert_eq!("point 3", last_points.last().and_then(|p| p.name.clone()).unwrap());
}

#[test]
fn test_split_route_zero() {
    let route = Route::new();

    let lim = crate::limit::PointsLimit::new(0);
    let split = RouteSplitter::new("".to_string(), lim);
    let tracks = split.spilt_routes(&vec![route]);
    assert_eq!(0, tracks.len());
}
