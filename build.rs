use std::env;
use copy_to_output::copy_to_output;

fn main() {
    println!("cargo:rerun-if-changed=tests/res/*");
    copy_to_output("tests/res/test.gpx", &env::var("PROFILE").unwrap()).expect("Could not copy");
}