//import
mod split;

//bring into scope
use split::Context;
use split::LengthSplitter;
use split::PointsSplitter;

fn main() {
    let mut a: Context<_> = Default::default();
    a.strategy = LengthSplitter::new(200);
    a.execute();

    let mut b: Context<_> = Default::default();
    b.strategy = PointsSplitter::new(50);
    b.execute();
}
