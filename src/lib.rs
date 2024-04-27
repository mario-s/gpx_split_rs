#![doc = include_str!("../README.md")]

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
