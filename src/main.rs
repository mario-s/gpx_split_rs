//import
use clap::{Parser, ValueEnum};

use gpx_split::split::Context;
use gpx_split::limit::{Limit, LengthLimit, PointsLimit};


/// program to split a GPX file
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// path of the file to split
    #[arg(value_name="PATH_TO_FILE")]
    path: String,
    /// track will be split when the maximum is exceeded, points or length in Meter
    #[arg(short, long, value_name="MAXIMUM", default_value_t = 500)]
    max: u32,
    /// split the track by number of points, or by length in Meter
    #[arg(short, long, value_enum, default_value_t=SplitType::Point)]
    split_type: SplitType,
}

/// splitting occurs when one of the maximum values is reached
#[derive(ValueEnum, Clone)]
enum SplitType {
    /// split by number of points
    Point,
    /// split by length of track
    Len,
}

fn main() {
    let args = Arguments::parse();
    let path = args.path;
    let split = args.split_type;
    let max = args.max;

    match split {
        SplitType::Len => execute(path, LengthLimit::new(max)),
        SplitType::Point => execute(path, PointsLimit::new(max)),
    }
}

fn execute<S>(path: String, strategy: S) where S: Limit + Clone {
    let mut c = Context::new(path, strategy);
    c.execute().expect("failed to spilt file!");
}
