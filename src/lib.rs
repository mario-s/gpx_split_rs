//!
//! This tool can split a route or track in a GPX file into smaller chunks.
//! It will create new files, where each one has a suffix to the filename like so FILENAME_1.gpx.
//!
//! <hr/>
//!
//! <p>The type of splitting is controlled with command line arguments. Here are some samples to cut a track or route.
//! <ul>
//! <li>split a track after 500 points (default)<br/>
//! <code>gpx_split My_Track.gpx</code></li>
//! <li>split a route after 600 points<br/>
//! <code>gpx_split -t route My_Route.gpx -m 600</code></li>
//! <li>split a track after 50 km (unit is meter)<br/>
//! <code>gpx_split My_Track.gpx -b len -m 50000</code></li>
//! <li>split a route after 50 km<br/>
//! <code>gpx_split -t route My_Route.gpx -b len -m 50000</code></li>
//! <li>split a track at some locations with a maximum distance to a POI of 20 m (if argument n is not provided, the program will look for splitting points in the source file)<br/>
//! <code>gpx_split My_Track.gpx -b loc -m 20 -n POI.gpx</code></li>
//! <li>split a route at some locations with a maximum distance to a POI of 10 m<br/>
//! <code>gpx_split -t route My_Route.gpx -b loc -m 10 -n POI.gpx</code></li>
//! </ul>
//! </p>

mod geo;
/// Input and output functions.
pub mod io;
/// Conditions when a limit is exceed and the track or route needs to be splitted.
pub mod limit;
/// The splitting logic for a track or route.
pub mod split;

/// Provides functions to handle geographic locations.
pub mod loc {
    pub use crate::geo::*;
}
