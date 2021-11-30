use std::path::Path;

use crate::config::model::*;
use crate::config::postman;
use crate::config::toml;

#[derive(Debug)]
pub enum Error {
    TomlError(toml::Error)
}

trait ConfigParser {
    fn from_string<S: AsRef<str>>(json: S) -> Result<Config, String>;
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let config = toml::from_file(&path)?;
    let config = match config.import {
        None => config,
        Some(ref imports) => {
            let children = imports.into_iter()
                .map(from_import)
                .filter_map(|import| {
                    match import {
                        Ok(config) => Some(config),
                        Err(e) => {
                            println!("{}", e);
                            None
                        }
                    }
                })
                .collect();
            Config {
                children: Some(children),
                ..config
            }
        }
    };
    Ok(config)
}

fn from_import(import: &Import) -> Result<Config, postman::Error> {
    match import.import_type {
        ImportType::Postman => postman::from_file(&import.path)
    }
}

impl From<toml::Error> for Error {
    fn from(err: toml::Error) -> Self {
        Error::TomlError(err)
    }
}
