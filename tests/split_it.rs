use gpx_split::split::Context;
use gpx_split::split::LengthSplitter;
use gpx_split::split::PointsSplitter;


#[test]
fn test_length() {
    let mut c = Context {
        path: "target/debug/test_l.gpx".to_string(),
        strategy: LengthSplitter::new(1000),
    };
    let res = c.execute().unwrap();
    assert_eq!(3, res)
}

#[test]
fn test_points() {
    let mut c = Context {
        path: "target/debug/test_p.gpx".to_string(),
        strategy: PointsSplitter::new(50),
    };
    let res = c.execute().unwrap();
    assert_eq!(2, res)
}