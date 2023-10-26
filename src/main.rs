//import
extern crate gpx_split;

//bring into scope
use gpx_split::Context;
use gpx_split::PointsSplitter;


fn main() {
    let s = PointsSplitter::new(50);
    let mut c = Context {
        file: "tests/test.gpx",
        strategy: s,
    };
    c.execute();
}
