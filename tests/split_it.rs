extern crate gpx_split;

use gpx_split::Context;
use gpx_split::LengthSplitter;
use gpx_split::PointsSplitter;

#[test]
fn test_length() {
    let s = LengthSplitter::new(200);
    let mut c = Context {
        file: "tests/test.gpx",
        strategy: s,
    };
    c.execute();
}

#[test]
fn test_points() {
    let s = PointsSplitter::new(500);
    let mut c = Context {
        file: "tests/test.gpx",
        strategy: s,
    };
    c.execute();
}