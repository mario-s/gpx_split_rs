mod geo;
mod io;
pub mod limit;
pub mod split;

/// A prelude which re-exports the traits for manipulating objects in this
/// crate. Typically imported with `use gpx_split::prelude::*`.
pub mod prelude {
    pub use crate::geo::*;
}
