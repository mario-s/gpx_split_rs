use split::Context;
use split::LengthSplitter;

#[test]
fn test_lengthsplitter() {
    let s = LengthSplitter::new(200);
    let mut c = Context {
        file: "",
        strategy: s,
    };
    c.execute();
}