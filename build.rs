use std::env;
use copy_to_output::copy_to_output;

fn main() {
    const KEY: &str = "PROFILE";
    const MSG: &str = "Could not copy";
    println!("cargo:rerun-if-changed=tests/res/*");
    copy_to_output("tests/res/track_l.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/track_p.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/route_l.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/route_p.gpx", &env::var(KEY).unwrap()).expect(MSG);
    copy_to_output("tests/res/pois.gpx", &env::var(KEY).unwrap()).expect(MSG);
}