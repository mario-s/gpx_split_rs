use gpx::{Gpx, Route, Track, TrackSegment, Waypoint};
use log::debug;
use std::io::Error;
use std::thread;
use std::thread::JoinHandle;

use crate::geo::fit_bounds;
use crate::io::*;
use crate::limit::Limit;

///common context for splitters
pub struct Context<T> {
    input_file: String,
    output_file: Option<String>,
    splitter: Box<dyn Splitter<T>>,
}

impl<T> Context<T> {
    pub fn new(
        input_file: String,
        output_file: Option<String>,
        splitter: Box<dyn Splitter<T>>,
    ) -> Self {
        Context {
            input_file,
            output_file,
            splitter,
        }
    }

    pub fn run(&self) -> Result<usize, Error> {
        let gpx = read_gpx(self.input_file.as_str())?;
        let origin = self.splitter.traces(gpx.clone());
        let len = origin.len();
        let new_traces = self.splitter.split(&origin);
        if new_traces.len() > len {
            debug!("{} traces after splitting", new_traces.len());
            return self.write(gpx, new_traces);
        }
        Ok(len)
    }

    fn write(&self, gpx: Gpx, traces: Vec<T>) -> Result<usize, Error> {
        let mut handles = Vec::new();
        let path = self.output_file.clone().unwrap_or(self.input_file.clone());

        traces.iter().enumerate().for_each(|(index, trace)| {
            let h = self.splitter.write(&path, &gpx, trace, index);
            handles.push(h);
        });

        for handle in handles {
            handle.join().unwrap()?;
        }

        Ok(traces.len())
    }
}

//--------------------------------------------------------------

/// Trait for impl which split a route or track.
///
pub trait Splitter<T> {
    fn traces(&self, gpx: Gpx) -> Vec<T>;
    fn split(&self, origin: &[T]) -> Vec<T>;
    fn write(
        &self,
        path: &str,
        gpx: &Gpx,
        trace: &T,
        counter: usize,
    ) -> JoinHandle<Result<(), Error>>;
}

/// Splitter for routes.
///
pub struct RouteSplitter {
    limit: Limit,
}

/// Splitter for tracks.
///
pub struct TrackSplitter {
    limit: Limit,
}

//--------------------------------------------------------------

impl Splitter<Route> for RouteSplitter {
    fn traces(&self, gpx: Gpx) -> Vec<Route> {
        gpx.routes
    }

    /// splits the given routes into new routes where the number of points of that route are limted
    ///
    fn split(&self, routes: &[Route]) -> Vec<Route> {
        let mut new_routes = Vec::new();
        let mut points = Vec::new();
        routes.iter().for_each(|route| {
            route.points.iter().for_each(|point| {
                points.push(point.clone());

                if self.limit.exceeds(&points) {
                    let new_route = self.clone_route(route, &points);
                    new_routes.push(new_route);

                    //clear points, starting with last one
                    points = clear_points(&points);
                }
            });
        });

        //this condition will be true in most cases
        //but it can happen that we split at the end of a route, in this case we have only one point
        if points.len() > 1 {
            if let Some(last) = routes.last() {
                let new_route = self.clone_route(last, &points);
                new_routes.push(new_route);
            }
        }

        new_routes
    }

    /// Writes the given route(s) into new files, when there are more than one route.
    /// If there is only one route, we did not split anything, so no need to write.
    ///
    fn write(
        &self,
        path: &str,
        gpx: &Gpx,
        route: &Route,
        index: usize,
    ) -> JoinHandle<Result<(), Error>> {
        let path = path.to_string();
        let gpx = gpx.clone();
        let mut route = route.clone();
        thread::spawn(move || {
            let mut gpx = fit_bounds(gpx, &route.points);
            route.name = append_index_to_name(route.name, index);
            gpx.routes.clear();
            gpx.routes.push(route);
            write_gpx(gpx, &path, index)
        })
    }
}

impl RouteSplitter {
    pub fn new(limit: Limit) -> Self {
        RouteSplitter { limit }
    }

    fn clone_route(&self, src_route: &Route, points: &[Waypoint]) -> Route {
        let mut cloned_route = src_route.clone();
        cloned_route.points = points.to_vec();
        cloned_route
    }
}

//--------------------------------------------------------------

impl Splitter<Track> for TrackSplitter {
    fn traces(&self, gpx: Gpx) -> Vec<Track> {
        gpx.tracks
    }

    /// splits the given tracks into new tracks where the number of points of that tracks are limted
    ///
    fn split(&self, tracks: &[Track]) -> Vec<Track> {
        let mut new_tracks = Vec::new();
        let mut points = Vec::new();
        tracks.iter().for_each(|track| {
            track
                .segments
                .iter()
                .flat_map(|segment| segment.points.iter())
                .for_each(|point| {
                    points.push(point.clone());

                    //create a new track when the points exceed a limit
                    if self.limit.exceeds(&points) {
                        let new_track = self.clone_track(track, &points);
                        new_tracks.push(new_track);

                        //clear points, starting with last one
                        points = clear_points(&points);
                    }
                });
        });
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

    /// Writes the given tracks into new files, when there are more than one route.
    /// If there is only one track, we did not split anything, so no need to write.
    ///
    fn write(
        &self,
        path: &str,
        gpx: &Gpx,
        track: &Track,
        index: usize,
    ) -> JoinHandle<Result<(), Error>> {
        let path = path.to_string();
        let gpx = gpx.clone();
        let mut track = track.clone();
        thread::spawn(move || {
            let points: Vec<Waypoint> = track
                .segments
                .iter()
                .flat_map(|s| s.points.iter().cloned())
                .collect();
            let mut gpx = fit_bounds(gpx, &points);
            track.name = append_index_to_name(track.name, index);
            gpx.tracks.clear();
            gpx.tracks.push(track);
            gpx.tracks.shrink_to_fit();

            write_gpx(gpx, &path, index)
        })
    }
}

impl TrackSplitter {
    pub fn new(limit: Limit) -> Self {
        TrackSplitter { limit }
    }

    /// clone the source track and add new track segment with the points
    ///
    fn clone_track(&self, src_track: &Track, points: &[Waypoint]) -> Track {
        let mut track_segment = TrackSegment::new();
        track_segment.points = points.to_vec();

        let mut cloned_track = src_track.clone();
        cloned_track.segments = vec![track_segment];
        cloned_track
    }
}

//--------------------------------------------------------------

// clear the points and add the previous last one as the first
fn clear_points(points: &Vec<Waypoint>) ->  Vec<Waypoint>{
    if let Some(last) = points.last() {
        return vec![last.clone()];
    }
    vec![]
}

//--------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::limit::Limit;
    use crate::split::{RouteSplitter, Splitter, TrackSplitter};
    use gpx::{Route, Track, TrackSegment, Waypoint};

    #[test]
    fn test_split_route_0() {
        let route = Route::new();

        let routes = new_route_splitter(0).split(&[route]);

        assert_eq!(0, routes.len());
    }

    #[test]
    fn test_split_route_1() {
        let route = new_route(4);

        let routes = new_route_splitter(4).split(&[route]);

        assert_eq!(1, routes.len());
    }

    #[test]
    fn test_split_route_3() {
        let route = new_route(4);

        let routes = new_route_splitter(2).split(&[route]);

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

    fn new_route_splitter(max: u32) -> Box<dyn Splitter<Route>> {
        let lim = Limit::points(max);
        Box::new(RouteSplitter::new(lim))
    }

    //--------------------------------------------------------------

    #[test]
    fn test_split_track_0() {
        let track = Track::new();

        let tracks = new_track_splitter(0).split(&[track]);

        assert_eq!(0, tracks.len());
    }

    #[test]
    fn test_split_track_1() {
        let track = new_track(4);

        let tracks = new_track_splitter(4).split(&[track]);

        assert_eq!(1, tracks.len());
    }

    #[test]
    fn test_split_track_3() {
        let track = new_track(4);

        let tracks = new_track_splitter(2).split(&[track]);

        //expect 2 tracks with 1 segment each containing 2 points
        assert_eq!(3, tracks.len());

        let first_points = tracks
            .first()
            .and_then(|t| t.segments.first())
            .map(|s| s.points.clone())
            .unwrap();
        let middle_points = tracks
            .get(1)
            .and_then(|t| t.segments.first())
            .map(|s| s.points.clone())
            .unwrap();
        let last_points = tracks
            .last()
            .and_then(|t| t.segments.first())
            .map(|s| s.points.clone())
            .unwrap();
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

    fn new_track_splitter(max: u32) -> Box<dyn Splitter<Track>> {
        let lim = Limit::points(max);
        Box::new(TrackSplitter::new(lim))
    }

    fn assert_points(
        first_points: Vec<Waypoint>,
        middle_points: Vec<Waypoint>,
        last_points: Vec<Waypoint>,
    ) {
        assert_eq!(2, first_points.len());
        assert_eq!(2, middle_points.len());
        assert_eq!(2, last_points.len());

        //first from 0 to 1
        assert_eq!(
            "point 0",
            first_points.first().and_then(|p| p.name.clone()).unwrap()
        );
        assert_eq!(
            "point 1",
            first_points.last().and_then(|p| p.name.clone()).unwrap()
        );
        //second from 1 to 2
        assert_eq!(
            "point 1",
            middle_points.first().and_then(|p| p.name.clone()).unwrap()
        );
        assert_eq!(
            "point 2",
            middle_points.last().and_then(|p| p.name.clone()).unwrap()
        );
        //third from 2 to 3
        assert_eq!(
            "point 2",
            last_points.first().and_then(|p| p.name.clone()).unwrap()
        );
        assert_eq!(
            "point 3",
            last_points.last().and_then(|p| p.name.clone()).unwrap()
        );
    }
}
