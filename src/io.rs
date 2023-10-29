use gpx::Gpx;
use gpx::errors::GpxError;
use gpx::read;

use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;

/// Function to convert an GpxError
pub fn to_error(gpx_err: GpxError) -> Error {
    Error::new(ErrorKind::Other, gpx_err.to_string())
}

pub fn read_gpx(path: &str) -> Result<Gpx, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match read(reader) {
        Ok(gpx) => Ok(gpx),
        Err(gpx_err) => Err(to_error(gpx_err))
    }
}