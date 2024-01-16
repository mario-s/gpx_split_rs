use gpx::errors::GpxError;
use gpx::read;
use gpx::write;
use gpx::Gpx;
use log::debug;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

/// Reads Gpx data from the given path.
pub fn read_gpx(path: &str) -> Result<Gpx, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match read(reader) {
        Ok(gpx) => Ok(gpx),
        Err(gpx_err) => Err(to_error(gpx_err)),
    }
}

/// Writes the Gpx into a new file pased on the given path
/// while appending the counter to the filename.
pub fn write_gpx(mut gpx: Gpx, path: &str, counter: usize) -> Result<(), Error> {
    gpx = update_metadata_name(gpx, counter);
    let path = create_path(path, counter)?;
    let file = File::create(&path)?;
    let res = write(&gpx, file);
    match res {
        Ok(_) => {
            debug!("wrote file {}", path);
            Ok(())
        }
        Err(gpx_err) => Err(to_error(gpx_err)),
    }
}

fn update_metadata_name(mut gpx: Gpx, counter: usize) -> Gpx {
    gpx.metadata = gpx.metadata.map(|mut meta| {
        meta.name = append_index(meta.name, counter);
        meta
    });
    gpx
}

/// Appends an integer to the end of the name.
pub fn append_index(name: Option<String>, index: usize) -> Option<String> {
    name.map(|n| format!("{} #{}", n, index))
}

/// creates a new path to a file
fn create_path(path: &str, counter: usize) -> Result<String, Error> {
    let parts: Vec<&str> = path.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("invalid file: {}", path),
        ));
    }
    //new file name would be like foo_1.gpx
    let name = format!("{}_{}.{}", parts[1], counter, parts[0]);
    Ok(name)
}

/// Function to convert an GpxError
fn to_error(gpx_err: GpxError) -> Error {
    Error::new(ErrorKind::InvalidData, gpx_err.to_string())
}

#[cfg(test)]
mod tests {
    use gpx::{Gpx, Metadata};

    use super::*;
    #[test]
    fn change_metadata_name() {
        let meta = Metadata {
            name: Some("bar".to_string()),
            ..Default::default()
        };
        let mut gpx: Gpx = Gpx {
            metadata: Some(meta),
            ..Default::default()
        };

        gpx = update_metadata_name(gpx, 1);
        let res = gpx.metadata.unwrap().name.unwrap();
        assert_eq!("bar #1", res);
    }

    #[test]
    fn create_path_with_counter() {
        let res = create_path("foo/bar.gpx", 1).unwrap();
        assert_eq!("foo/bar_1.gpx", res);
    }
}
