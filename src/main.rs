use clap::{Parser, ValueEnum};
use log::debug;
use std::io::Error;
use std::time::Instant;

use gpx_split::limit::{LengthLimit, Limit, PointsLimit};
use gpx_split::split::{Context, RouteSplitter, Splitter, TrackSplitter};

/// A program to split a GPX file into smaller chunks
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path of the file to split
    #[arg(value_name = "PATH_TO_FILE")]
    path: String,
    /// Track/route will be split, when the maximum is exceeded, points or length in Meter
    #[arg(short, long, value_name = "MAXIMUM", default_value_t = 500)]
    max: u32,
    /// Objects to split: either routes or the tracks in the GPX file
    #[arg(short, long, value_enum, default_value_t=Trace::Track)]
    trace: Trace,
    /// Method to split the object: by number of points, or by length in Meter
    #[arg(short, long, value_enum, default_value_t=By::Point)]
    by: By,
    /// Path for output file, e.g. foo/bar.gpx. The program creates then foo/bar_0.gpx, foo/bar_1.gpx and so on
    #[arg(short, long)]
    output: Option<String>,
}

/// what to split in the gpx file
#[derive(ValueEnum, Clone)]
enum Trace {
    /// split the routes
    Route,
    /// split the tracks
    Track,
}

/// splitting occurs when one of the maximum values is reached
#[derive(ValueEnum, Clone)]
enum By {
    /// split by number of points
    Point,
    /// split by length of track
    Len,
}

fn main() {
    env_logger::init();
    let now = Instant::now();

    let args = Arguments::parse();
    let path = args.path;
    let trace = args.trace;
    let by = args.by;
    let max = args.max;
    let out = args.output;

    let limit = || {create_limit(max, by)};
    let res = match trace {
        Trace::Route => run(path.clone(), out, Box::new(RouteSplitter::new(limit()))),
        Trace::Track => run(path.clone(), out, Box::new(TrackSplitter::new(limit()))),
    };
    res.unwrap();

    debug!("Splitting took {} microseconds.", now.elapsed().as_micros());
}

fn create_limit(max: u32, by: By) -> Box<dyn Limit> {
    match by {
        By::Len => Box::new(LengthLimit::new(max)),
        By::Point => Box::new(PointsLimit::new(max)),
    }
}

fn run<T: 'static>(
    path: String,
    output: Option<String>,
    splitter: Box<dyn Splitter<T>>,
) -> Result<usize, Error> {
    let c = Context::new(path, output, splitter);
    c.run()
}
