use crate::config::errors::MapErrWithContext;
use crate::config::model::{Config, Import, ImportType};
use crate::config::toml::from_file as from_toml_file;
use crate::config::postman::from_file as from_postman_file;
use crate::config::errors::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn load(path: &str) -> Result<Config> {
    let original_config = from_toml_file(path)
        .map_err_with_context(|| format!("Error loading: {}", path))
        .map(|config| {
            Config {
                path: Some(path.to_string()),
                ..config
            }
        })?;

    let config = match original_config.import {
        None => original_config,
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
                ..original_config
            }
        }
    };
    Ok(config)
}

fn from_import(import: &Import) -> Result<Config> {
    match import.import_type {
        ImportType::Postman =>
            from_postman_file(import.path.as_str())
                .map_err_with_context(|| format!("Error importing postman file: {}", import.path))
    }
}

