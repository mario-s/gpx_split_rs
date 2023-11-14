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

/// creates a new path to a file
///
pub fn create_path(path: &String, counter: usize) -> Result<String, Error> {
    let parts: Vec<&str> = path.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, format!("invalid file: {}", path)));
    }
    //new file name would be like foo_1.gpx
    let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
    Ok(name)
}

#[test]
fn test_create_path() {
    let res = create_path(&"foo/bar.gpx".to_string(), 1).unwrap();
    assert_eq!("foo/bar_1.gpx", res);
}