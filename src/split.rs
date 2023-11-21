use std::io::Error;
use log::info;
use gpx::{Gpx, Route, Track, TrackSegment, Waypoint};

use crate::limit::Limit;
use crate::io::*;
use crate::geo::fit_bounds;

/// Trait for impls which split a route or track.
///
pub trait Splitter {
    fn split(&self) -> Result<usize, Error>;
}

/// Splitter for routes.
///
pub struct RouteSplitter {
    path: String,
    limit: Box<dyn Limit>,
}

/// Splitter for tracks.
///
pub struct TrackSplitter {
    path: String,
    limit: Box<dyn Limit>,
}

//--------------------------------------------------------------

impl Splitter for RouteSplitter {

    fn split(&self) -> Result<usize, Error> {
        let gpx = read_gpx(self.path.as_str())?;
        let existing = &gpx.routes;
        let routes = self.spilt_routes(existing);
        if routes.len() > existing.len() {
            info!("{} routes after splitting", routes.len());
            return self.write_routes(gpx, &routes);
        }
        Ok(existing.len())
    }
}

impl RouteSplitter {

    pub fn new(path: String, limit: Box<dyn Limit>) -> Self {
        RouteSplitter { path, limit }
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
        //this condition will be true in most cases
        //but it can happen that we split at the end of a route, in this case we have only one point
        if points.len() > 1 {
            if let Some(last) = routes.last() {
                let new_route = self.clone_route(last, &mut points);
                new_routes.push(new_route);
            }
        }

        new_routes
    }

    fn clone_route(&self, src_route: &Route, points: &mut Vec<Waypoint>) -> Route {
        let mut cloned_route = src_route.clone();
        cloned_route.points.clear();
        cloned_route.points.append(points);
        cloned_route.points.shrink_to_fit();
        cloned_route
    }

    /// Writes the given route(s) into new files, when there are more than one route.
    /// If there is only one route, we did not split anything, so no need to write.
    ///
    fn write_routes(&self, src_gpx: Gpx, routes: &Vec<Route>) -> Result<usize, Error> {
        for (index, route) in routes.iter().enumerate() {
            self.write_route(&src_gpx, route, index)?;
        }

        Ok(routes.len())
    }

    /// writes a single route into a file, counter is the suffix for the file name
    ///
    fn write_route(&self, src_gpx: &Gpx, route: &Route, counter: usize) -> Result<(), Error> {
        //clone the source gpx and just clear the tracks to keep the rest
        let mut gpx = fit_bounds(src_gpx.clone(), &route.points);
        gpx.routes.clear();
        gpx.routes.push(route.to_owned());

        write_gpx(gpx, &self.path, counter)
    }
}

//--------------------------------------------------------------

impl Splitter for TrackSplitter {
    fn split(&self) -> Result<usize, Error> {
        let gpx = read_gpx(self.path.as_str())?;
        let existing = &gpx.tracks;
        let tracks = self.spilt_tracks(&existing);
        if tracks.len() > existing.len() {
            info!("{} routes after splitting", tracks.len());
            return self.write_tracks(gpx, tracks);
        }
        Ok(existing.len())
    }
}

impl TrackSplitter {

    pub fn new(path: String, limit: Box<dyn Limit>) -> Self {
        TrackSplitter { path, limit }
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
        //but it can happen that we split at the end of a track, in this case we have only one point
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
        cloned_track.segments.shrink_to_fit();

        cloned_track
    }

    /// Writes the given tracks into new files, when there are more than one route.
    /// If there is only one track, we did not split anything, so no need to write.
    ///
    fn write_tracks(&self, src_gpx: Gpx, tracks: Vec<Track>) -> Result<usize, Error> {
        for (index, track) in tracks.iter().enumerate() {
            self.write_track(&src_gpx, track, index)?;
        }

        Ok(tracks.len())
    }

    /// writes a single track into a file, counter is the suffix for the file name
    ///
    fn write_track(&self, src_gpx: &Gpx, track: &Track, counter: usize) -> Result<(), Error> {
        //clone the source gpx and just clear the tracks to keep the rest
        let points:Vec<Waypoint> = track.segments.iter().flat_map(|s| s.points.iter().cloned()).collect();
        let mut gpx = fit_bounds(src_gpx.clone(), &points);
        gpx.tracks.clear();
        gpx.tracks.push(track.to_owned());
        gpx.tracks.shrink_to_fit();

        write_gpx(gpx, &self.path, counter)
    }
}

//--------------------------------------------------------------

#[cfg(test)]
mod tests {
    use gpx::{Route, Track, TrackSegment, Waypoint};
    use crate::limit::PointsLimit;
    use crate::split::{RouteSplitter, TrackSplitter};

    #[test]
    fn test_split_route_0() {
        let route = Route::new();

        let routes = new_route_splitter(0).spilt_routes(&vec![route]);

        assert_eq!(0, routes.len());
    }

    #[test]
    fn test_split_route_1() {
        let route = new_route(4);

        let routes = new_route_splitter(4).spilt_routes(&vec![route]);

        assert_eq!(1, routes.len());
    }

    #[test]
    fn test_split_route_3() {
        let route = new_route(4);

        let routes = new_route_splitter(2).spilt_routes(&vec![route]);

        assert_eq!(3, routes.len());
        let first_points = routes.first().map(|r| r.points.clone()).unwrap();
        let middle_points = routes.get(1).map(|r| r.points.clone()).unwrap();
        let last_points = routes.last().map(|r| r.points.clone()).unwrap();
        assert_points(first_points, middle_points, last_points);
    }

    fn new_route(num_points: u32) -> Route {
        let mut route = Route::new();
        for i in 0..num_points {
            let mut point = Waypoint::default();
            point.name = Some(format!("point {}", i));
            route.points.push(point);
        }
        route
    }

    fn new_route_splitter(max: u32) -> RouteSplitter {
        let lim = Box::new(PointsLimit::new(max));
        RouteSplitter::new("".to_string(), lim)
    }

    //--------------------------------------------------------------

    #[test]
    fn test_split_track_0() {
        let track = Track::new();

        let tracks = new_track_splitter(0).spilt_tracks(&vec![track]);

        assert_eq!(0, tracks.len());
    }

    #[test]
    fn test_split_track_1() {
        let track = new_track(4);

        let tracks = new_track_splitter(4).spilt_tracks(&vec![track]);

        assert_eq!(1, tracks.len());
    }

    #[test]
    fn test_split_track_3() {
        let track = new_track(4);

        let tracks = new_track_splitter(2).spilt_tracks(&vec![track]);

        //expect 2 tracks with 1 segment each containing 2 points
        assert_eq!(3, tracks.len());

        let first_points = tracks.first()
            .and_then(|t| t.segments.first()).map(|s| s.points.clone()).unwrap();
        let middle_points = tracks.get(1)
            .and_then(|t| t.segments.first()).map(|s| s.points.clone()).unwrap();
        let last_points = tracks.last()
            .and_then(|t| t.segments.first()).map(|s| s.points.clone()).unwrap();
        assert_points(first_points, middle_points, last_points);
    }

    fn new_track(num_points: u32) -> Track {
        let mut segment = TrackSegment::new();
        for i in 0..num_points {
            let mut point = Waypoint::default();
            point.name = Some(format!("point {}", i));
            segment.points.push(point);
        }
        let mut track = Track::new();
        track.segments.push(segment);
        track
    }

    fn new_track_splitter(max: u32) -> TrackSplitter {
        let lim = Box::new(PointsLimit::new(max));
        TrackSplitter::new("".to_string(), lim)
    }

    fn assert_points(first_points: Vec<Waypoint>, middle_points: Vec<Waypoint>, last_points: Vec<Waypoint>) {
        assert_eq!(2, first_points.len());
        assert_eq!(2, middle_points.len());
        assert_eq!(2, last_points.len());

        //first from 0 to 1
        assert_eq!("point 0", first_points.first().and_then(|p| p.name.clone()).unwrap());
        assert_eq!("point 1", first_points.last().and_then(|p| p.name.clone()).unwrap());
        //second from 1 to 2
        assert_eq!("point 1", middle_points.first().and_then(|p| p.name.clone()).unwrap());
        assert_eq!("point 2", middle_points.last().and_then(|p| p.name.clone()).unwrap());
        //third from 2 to 3
        assert_eq!("point 2", last_points.first().and_then(|p| p.name.clone()).unwrap());
        assert_eq!("point 3", last_points.last().and_then(|p| p.name.clone()).unwrap());
    }
}