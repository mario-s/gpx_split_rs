//import
mod split;

//bring into scope
use split::Context;
use split::LengthSplitter;
use split::PointsSplitter;

fn main() {
    /// PointsSplitter::new(50)
    let s = LengthSplitter::new(200);
    let mut c = Context {
        file: "",
        strategy: s,
    };
    c.execute();
}
