use gpx_split::split::{Splitter, TrackSplitter};
use gpx_split::limit::{LengthLimit, PointsLimit};

#[test]
fn test_length() {
    let s = TrackSplitter::new(
        "target/debug/test_l.gpx".to_string(),
        Box::new(LengthLimit::new(1000)));

    assert_splitted(s, 3)
}

#[test]
fn test_points() {
    let s = TrackSplitter::new(
        "target/debug/test_p.gpx".to_string(),
        Box::new(PointsLimit::new(50)));
    assert_splitted(s, 2)
}

fn assert_splitted<T: Splitter>(splitter: T, size: usize) {
    let res = splitter.split().unwrap();
    assert_eq!(size, res)
}