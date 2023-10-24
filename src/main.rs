//import
extern crate gpx_split;

//bring into scope
use gpx_split::Context;
use gpx_split::LengthSplitter;


fn main() {
    /// PointsSplitter::new(50)
    let s = LengthSplitter::new(200);
    let mut c = Context {
        file: "",
        strategy: s,
    };
    c.execute();
}
