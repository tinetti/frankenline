use std::path::Path;
use crate::error::Result;

pub mod model;
mod loader;
mod postman;
mod toml;

pub fn load<P: AsRef<Path>>(path: P) -> Result<model::Config> {
    let config = loader::load(&path)?;
    Ok(config)
}
