use std::env;
use copy_to_output::copy_to_output;

fn main() {
    const KEY: &str = "PROFILE";
    const MSG: &str = "Could not copy";
    println!("cargo:rerun-if-changed=tests/res/*");
    copy_to_output("tests/res/track_len.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/track_points.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/track_loc.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/route_len.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/route_points.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/pois.gpx", &env::var(KEY).unwrap()).expect(MSG);
}