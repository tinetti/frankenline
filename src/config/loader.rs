use std::fs;
use std::path::Path;

use crate::error::{Error, Result};
use crate::config::model::*;
use crate::config::postman::PostmanConfigParser;
use crate::config::toml::TomlConfigParser;


pub trait ConfigParser<S: AsRef<str>> {
    fn parse(&self, text: S) -> Result<Config>;
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config = parse_config_file(path)?;
    let config = match config.import {
        None => config,
        Some(ref imports) => {
            let children = imports.into_iter()
                .map(|import| load(&import.path))
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

fn find_parser<P: AsRef<Path>, S: AsRef<str>>(path: P) -> Box<dyn ConfigParser<S>> {
    let extension = path.as_ref().extension();
    let extension= extension.map_or(None, |ext| ext.to_str());
    match extension {
        Some("json") => Box::new(PostmanConfigParser{}),
        Some(_) | None => Box::new(TomlConfigParser{}),
    }
}

fn parse_config_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let parser = find_parser(&path);
    let parser = parser.as_ref();
    let text = fs::read_to_string(&path)
        .map_err(|err| Error::new(format!("Unable to load config file '{}': {}", path.as_ref().display(), err)))?;
    let mut config = parser.parse(text)?;
    config.path = Some(path.as_ref().to_path_buf());
    Ok(config)
}
