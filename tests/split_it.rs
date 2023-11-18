use gpx_split::split::{Splitter, TrackSplitter, RouteSplitter};
use gpx_split::limit::{LengthLimit, PointsLimit};

#[test]
fn test_track_length() {
    let s = TrackSplitter::new(
        "target/debug/track_l.gpx".to_string(),
        Box::new(LengthLimit::new(1000)));

    assert_splitted(s, 3)
}

#[test]
fn test_track_points() {
    let s = TrackSplitter::new(
        "target/debug/track_p.gpx".to_string(),
        Box::new(PointsLimit::new(50)));
    assert_splitted(s, 2)
}

#[test]
fn test_route_length() {
    let s = RouteSplitter::new(
        "target/debug/route_l.gpx".to_string(),
        Box::new(LengthLimit::new(5000)));
    assert_splitted(s, 3)
}

#[test]
fn test_route_points() {
    let s = RouteSplitter::new(
        "target/debug/route_p.gpx".to_string(),
        Box::new(PointsLimit::new(40)));
    assert_splitted(s, 2)
}

fn assert_splitted<T: Splitter>(splitter: T, size: usize) {
    let res = splitter.split().unwrap();
    assert_eq!(size, res)
}