//import

use gpx_split::split::Context;
use gpx_split::split::LengthSplitter;
use gpx_split::split::PointsSplitter;
use gpx_split::split::Splitter;

fn main() {
    let s = PointsSplitter::new(50);
    execute("", s);

    let s = LengthSplitter::new(2500.0);
    execute("", s);
}

fn execute<S>(file: &'static str, strategy: S) where S: Splitter {
    let mut c = Context::new(file, strategy);
    c.execute().expect("failed to spilt file!");
}
