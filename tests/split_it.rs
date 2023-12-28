use gpx_split::split::{Context, TrackSplitter, RouteSplitter};
use gpx_split::limit::Limit;

#[test]
fn track_length() {
    let path = "target/debug/track_l.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::length(800)));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn track_points() {
    let path = "target/debug/track_p.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(Limit::points(50)));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}

#[test]
fn route_length() {
    let path = "target/debug/route_l.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::length(5000)));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn route_points() {
    let path = "target/debug/route_p.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(Limit::points(40)));

    let ctx = Context::new(path, None, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}
