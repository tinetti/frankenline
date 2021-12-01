use std::fs;
use std::path::Path;

use crate::config::error::Error;
use crate::config::model::*;
use crate::config::postman::PostmanConfigParser;
use crate::config::toml::TomlConfigParser;

pub trait ConfigParser {
    fn parse<S: AsRef<str>>(text: S) -> Result<Config, Error>;
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let config = load_toml_file(&path)?;
    let config = match config.import {
        None => config,
        Some(ref imports) => {
            let children = imports.into_iter()
                .map(from_import)
                .filter_map(|import| {
                    match import {
                        Ok(config) => Some(config),
                        Err(e) => {
                            println!("Warning: {}", e);
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

fn load_postman_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let text = fs::read_to_string(&path)
        .map_err(|err| Error::new(format!("Unable to load Postman file '{}': {}", path.as_ref().display(), err)))?;
    let mut config = PostmanConfigParser::parse(text)?;
    config.path = Some(path.as_ref().to_path_buf());
    Ok(config)
}

fn load_toml_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let text = fs::read_to_string(&path)
        .map_err(|err| Error::new(format!("Unable to load config file '{}': {}", path.as_ref().display(), err)))?;
    let mut config = TomlConfigParser::parse(text)?;
    config.path = Some(path.as_ref().to_path_buf());
    Ok(config)
}


fn from_import(import: &Import) -> Result<Config, Error> {
    match import.import_type {
        ImportType::Postman => load_postman_file(&import.path)
    }
}
