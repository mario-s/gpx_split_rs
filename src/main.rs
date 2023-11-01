//import
use clap::Parser;

use gpx_split::split::Context;
use gpx_split::split::LengthLimit;
use gpx_split::split::PointsLimit;
use gpx_split::split::Limit;

/// program to split a gpx file
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// path of the file to split
    #[arg(short, long)]
    file_path: String,
    /// type of splitting: either 'p' for points, or 'l' for length in Meter
    #[arg(short, long, default_value_t = 'p')]
    split_type: char,
    /// track will be split when the maximum is exceeded, points or length in Meter
    #[arg(short, long, default_value_t = 500)]
    max: u32,
}

fn main() {
    let args = Arguments::parse();
    let path = args.file_path;
    let split = args.split_type;
    let max = args.max;

    match split {
        'l' => execute(path, LengthLimit::new(max)),
        'p' => execute(path, PointsLimit::new(max)),
        _   => panic!("unknown split type: '{}'", split),
    }
}

fn execute<S>(path: String, strategy: S) where S: Limit {
    let mut c = Context::new(path, strategy);
    c.execute().expect("failed to spilt file!");
}
