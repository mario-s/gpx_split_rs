use gpx_split::split::TrackSplitter;
use gpx_split::limit::LengthLimit;
use gpx_split::limit::PointsLimit;


#[test]
fn test_length() {
    let s = TrackSplitter::new(
        "target/debug/test_l.gpx".to_string(),
        LengthLimit::new(1000));
    let res = s.split().unwrap();
    assert_eq!(3, res)
}

#[test]
fn test_points() {
    let s = TrackSplitter::new(
        "target/debug/test_p.gpx".to_string(),
        PointsLimit::new(50));
    let res = s.split().unwrap();
    assert_eq!(2, res)
}