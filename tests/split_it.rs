use gpx_split::split::{Context, TrackSplitter, RouteSplitter};
use gpx_split::limit::Limit;

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
}

#[test]
fn track_points() {
    let path = "target/debug/track_points.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::points(50)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}

#[test]
fn track_location() {
    let path = "target/debug/track_loc.gpx".to_string();
    let waypoints = "target/debug/pois.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::location(waypoints, 39)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn route_length() {
    let path = "target/debug/route_len.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::length(5000)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn route_points() {
    let path = "target/debug/route_points.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::points(40)));

    let mut ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}
