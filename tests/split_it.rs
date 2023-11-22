use gpx_split::split::{Context, TrackSplitter, RouteSplitter};
use gpx_split::limit::{LengthLimit, PointsLimit};

#[test]
fn test_track_length() {
    let path = "target/debug/track_l.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(path.clone(),
        Box::new(LengthLimit::new(1000))));

    let ctx = Context::new(path, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn test_track_points() {
    let path = "target/debug/track_p.gpx".to_string();
    let splitter = Box::new(TrackSplitter::new(path.clone(),
    Box::new(PointsLimit::new(50))));

    let ctx = Context::new(path, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}

#[test]
fn test_route_length() {
    let path = "target/debug/route_l.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(path.clone(),
    Box::new(LengthLimit::new(5000))));

    let ctx = Context::new(path, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(3, res);
}

#[test]
fn test_route_points() {
    let path = "target/debug/route_p.gpx".to_string();
    let splitter = Box::new(RouteSplitter::new(path.clone(),
    Box::new(PointsLimit::new(40))));

    let ctx = Context::new(path, splitter);
    let res = ctx.run().unwrap();

    assert_eq!(2, res);
}
