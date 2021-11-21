use std::error::Error;
use crate::config::model::{Config, Import, ImportType};
use crate::config::toml::from_file as from_toml_file;
use crate::config::postman::from_file as from_postman_file;

pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let mut original_config = from_toml_file(path)?;
    let config = match original_config.import {
        None => original_config,
        Some(ref imports) => {
            let mut commands = vec![];
            commands.append(&mut original_config.command);
            for import in imports {
                let mut child = from_import(import)?;
                commands.append(&mut child.command);
            }
            Config {
                command: commands,
                ..original_config
            }
        }
    };
    Ok(config)
}

fn from_import(import: &Import) -> Result<Config, Box<dyn Error>> {
    match import.import_type {
        ImportType::Postman => from_postman_file(import.path.as_str())
    }
}
