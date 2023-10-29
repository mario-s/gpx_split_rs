use gpx_split::split::Context;
use gpx_split::split::LengthSplitter;
use gpx_split::split::PointsSplitter;


#[test]
fn test_length() {
    let mut c = Context {
        file: "target/debug/test_l.gpx",
        strategy: LengthSplitter::new(1000.0),
    };
    match c.execute() {
        Ok(_) => Ok(()),
        Err(exc) => fail!(exc)
    }
}

#[test]
fn test_points() {
    let mut c = Context {
        file: "target/debug/test_p.gpx",
        strategy: PointsSplitter::new(50),
    };
    match c.execute() {
        Ok(_) => Ok(()),
        Err(exc) => fail!(exc)
    }
}