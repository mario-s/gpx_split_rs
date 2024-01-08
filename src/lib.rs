mod geo;
pub mod io;
pub mod limit;
pub mod split;

/// A prelude which re-exports the traits for manipulating objects in this
/// crate. Typically imported with `use gpx_split::prelude::*`.
pub mod loc {
    pub use crate::geo::*;
}
