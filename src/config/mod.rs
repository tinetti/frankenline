use std::path::Path;

pub mod error;
pub mod model;
mod loader;
mod postman;
mod toml;

pub fn load<P: AsRef<Path>>(path: P) -> Result<model::Config, error::Error> {
    let config = loader::load(&path)?;
    Ok(config)
}
