use std::io::Error;
use clap::{Parser, ValueEnum};

use gpx_split::split::{Splitter, TrackSplitter};
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
        SplitType::Len => build_and_run(path, LengthLimit::new(max)),
        SplitType::Point => build_and_run(path, PointsLimit::new(max)),
    }
}

fn build_and_run<L>(path: String, limit: L) where L: Limit + Clone {
    let s = TrackSplitter::new(path, limit);
    run(s).expect("failed to spilt file!");
}

fn run<T: Splitter>(splitter: T) -> Result<usize, Error> {
    splitter.split()
}