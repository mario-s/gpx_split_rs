use gpx::Gpx;
use gpx::errors::GpxError;
use gpx::read;
use gpx::write;
use log::debug;
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;

/// Reads Gpx data from the given path.
///
pub fn read_gpx(path: &str) -> Result<Gpx, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match read(reader) {
        Ok(gpx) => Ok(gpx),
        Err(gpx_err) => Err(to_error(gpx_err))
    }
}

/// Writes the Gpx into a new file pased on the given path
/// while appending the counter to the filename.
///
pub fn write_gpx(gpx: Gpx, path: &str, counter: usize) -> Result<(), Error> {
    let p = create_path(path, counter)?;
    let file = File::create(&p)?;
        let res = write(&gpx, file);
        match res {
            Ok(_) => {
                debug!("wrote file {}", p);
                Ok(())
            },
            Err(gpx_err) => Err(to_error(gpx_err))
        }
}

/// creates a new path to a file
///
fn create_path(path: &str, counter: usize) -> Result<String, Error> {
    let parts: Vec<&str> = path.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, format!("invalid file: {}", path)));
    }
    //new file name would be like foo_1.gpx
    let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
    Ok(name)
}

/// Function to convert an GpxError
fn to_error(gpx_err: GpxError) -> Error {
    Error::new(ErrorKind::Other, gpx_err.to_string())
}

#[test]
fn test_create_path() {
    let res = create_path("foo/bar.gpx", 1).unwrap();
    assert_eq!("foo/bar_1.gpx", res);
}