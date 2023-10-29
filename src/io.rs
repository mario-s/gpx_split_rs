use gpx::Gpx;
use gpx::errors::GpxError;
use gpx::read;
use gpx::write;

use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;

pub fn read_gpx(path: &str) -> Result<Gpx, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match read(reader) {
        Ok(gpx) => Ok(gpx),
        Err(gpx_err) => Err(to_error(gpx_err))
    }
}

pub fn write_gpx(gpx: Gpx, path: String) -> Result<(), Error> {
    let file = File::create(path)?;
        let res = write(&gpx, file);
        match res {
            Ok(_) => Ok(()),
            Err(gpx_err) => Err(to_error(gpx_err))
        }
}

/// Function to convert an GpxError
fn to_error(gpx_err: GpxError) -> Error {
    Error::new(ErrorKind::Other, gpx_err.to_string())
}