use gpx_split::split::{Context, TrackSplitter, RouteSplitter};
use gpx_split::limit;

#[test]
fn test_track_length() {
    let path = "target/debug/track_l.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(
        Box::new(limit::length(1000))));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn test_track_points() {
    let path = "target/debug/track_p.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(
    Box::new(limit::points(50))));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}

#[test]
fn test_route_length() {
    let path = "target/debug/route_l.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(
    Box::new(limit::length(5000))));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn test_route_points() {
    let path = "target/debug/route_p.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(
    Box::new(limit::points(40))));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}
