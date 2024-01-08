use gpx_split::split::{Context, TrackSplitter, RouteSplitter};
use gpx_split::limit::Limit;
use gpx_split::io::read_gpx;

#[test]
fn track_length_too_long() {
    let path = "target/debug/track_len.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::length(5000)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(0, res);
}

#[test]
fn track_length() {
    let path = "target/debug/track_len.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::length(800)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
    verify_track("target/debug/track_len", 3, 19);
}

#[test]
fn track_points() {
    let path = "target/debug/track_points.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::points(50)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
    verify_track("target/debug/track_points", 2, 50);
}

#[test]
fn track_location() {
    let path = "target/debug/track_loc.gpx".to_string();
    let waypoints = "target/debug/pois.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::location(waypoints, 39)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
    verify_track("target/debug/track_loc", 3, 280);
}

fn verify_track(pattern: &str, files: usize, min_points: usize) {
    for i in 0..files {
        let p = pattern.to_owned() + &format!("_{}.gpx", i);
        let gpx = read_gpx(&p).unwrap();
        let tracks = gpx.tracks;
        assert_eq!(1, tracks.len());
        let points = &tracks[0].segments[0].points;
        if i < files - 1 {
            assert!(points.len() >= min_points);
        } else {
            assert!(points.len() >= 2);
        }
    }
}

#[test]
fn route_length() {
    let path = "target/debug/route_len.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::length(5000)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
    verify_route("target/debug/route_len", 3, 18);
}

#[test]
fn route_points() {
    let path = "target/debug/route_points.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::points(40)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
    verify_route("target/debug/route_points", 2, 40);
}

fn verify_route(pattern: &str, files: usize, min_points: usize) {
    for i in 0..files {
        let p = pattern.to_owned() + &format!("_{}.gpx", i);
        let gpx = read_gpx(&p).unwrap();
        let tracks = gpx.routes;
        assert_eq!(1, tracks.len());
        let points = &tracks[0].points;
        if i < files - 1 {
            assert!(points.len() >= min_points);
        } else {
            assert!(points.len() >= 2);
        }
    }
}