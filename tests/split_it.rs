use gpx_split::split::Context;
use gpx_split::split::LengthSplitter;
use gpx_split::split::PointsSplitter;

const FILE: &str = "target/debug/test.gpx";

#[test]
fn test_length() {
    let s = LengthSplitter::new(1000.0);
    let mut c = Context {
        file: FILE,
        strategy: s,
    };
    c.execute();
}

#[test]
fn test_points() {
    let s = PointsSplitter::new(500);
    let mut c = Context {
        file: FILE,
        strategy: s,
    };
    c.execute();
}