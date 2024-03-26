use clap::{Parser, ValueEnum};
use log::debug;
use std::io::Error;
use std::process;
use std::time::Instant;

use gpx_split::limit::Limit;
use gpx_split::split::{Context, RouteSplitter, Splitter, TrackSplitter};

/// A program to split a GPX file into smaller chunks
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path of the file to split
    #[arg(value_name = "PATH_TO_FILE")]
    path: String,
    /// Track/route will be split, when the maximum is exceeded, points or distance in Meter
    #[arg(short, long, value_name = "MAXIMUM", default_value_t = 500)]
    max: u32,
    /// Objects to split: either routes or the tracks in the GPX file
    #[arg(short, long, value_enum, default_value_t=Trace::Track)]
    trace: Trace,
    /// Method to split the object: by number of points, length or location
    #[arg(short, long, value_enum, default_value_t=By::Point)]
    by: By,
    /// Path for output file, e.g. foo/bar.gpx. The program creates then foo/bar_0.gpx, foo/bar_1.gpx and so on
    #[arg(short, long)]
    output: Option<String>,
    /// Path to a file that contains the splitting points for a track/route. If not provided, the file to split will be used
    #[arg(short, long)]
    near: Option<String>,
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
    /// split by length
    Len,
    /// split by location
    Loc,
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
    let near = args.near;

    let limit = match by {
        By::Len => Limit::length(max),
        By::Point => Limit::points(max),
        By::Loc => Limit::location(&near.unwrap_or(path.clone()), max),
    };

    let res = match trace {
        Trace::Route => run(&path, out, Box::new(RouteSplitter::new(limit))),
        Trace::Track => run(&path, out, Box::new(TrackSplitter::new(limit))),
    }
    .unwrap_or_else(|err| {
        eprintln!("Problem processing GPX file: {err}");
        process::exit(1);
    });

    debug!(
        "Splitting source into {} files took {} microseconds.",
        res,
        now.elapsed().as_micros()
    );
}

fn run<T: 'static>(
    path: &str,
    output: Option<String>,
    splitter: Box<dyn Splitter<T>>,
) -> Result<usize, Error> {
    let mut c = Context::new(path.to_string(), output, splitter);
    c.run()
}
