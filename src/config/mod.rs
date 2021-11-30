use std::error;
use std::fmt;
use std::path::Path;

pub mod model;
mod loader;
mod postman;
mod toml;

#[derive(Debug)]
pub struct Error {
    cause: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error loading config")
    }
}

impl error::Error for Error {}

pub fn load<P: AsRef<Path>>(path: P) -> Result<model::Config, Error> {
    let config = loader::load(&path)
        .map_err(|err| Error { cause: format!("Error loading config from path: {} (caused by: {:?})", &path.as_ref().display(), err) })?;
    Ok(config)
}
