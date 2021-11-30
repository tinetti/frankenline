use std::fmt;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::config::model::{Command, Config};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let text = fs::read_to_string(&path)?;
    let config = from_string(text)?;
    let config = Config {
        path: Some(path.as_ref().to_path_buf()),
        ..config
    };
    Ok(config)
}

pub fn from_string<S: AsRef<str>>(json: S) -> Result<Config, Error> {
    serde_json::from_str::<Postman>(json.as_ref())
        .map(Config::from)
        .map_err(|err| err.into())
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(cause) => write!(f, "IO Error({})", cause),
            Error::Serde(cause) => write!(f, "Serde Error({})", cause),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

#[derive(Deserialize, Debug)]
struct Postman {
    info: Info,
    item: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Info {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Item {
    name: String,
}


impl From<Postman> for Config {
    fn from(postman: Postman) -> Self {
        let commands = postman.item.into_iter().map(|item| {
            Command {
                name: item.name,
            }
        }).collect();
        Config {
            description: format!("{} (Postman Collection)", postman.info.name),
            command: commands,
            import: None,
            path: None,
            children: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_postman_from_string() -> Result<(), Error> {
        let text = r#"
        {
            "info": {
                "_postman_id": "41e697c5-cadb-437b-b2e9-336c00724c8f",
                "name": "Frankenline",
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": [
                {
                    "name": "example - get json from api",
                    "request": {
                        "method": "GET",
                        "header": [],
                        "url": {
                            "raw": "https://dog-facts-api.herokuapp.com/api/v1/resources/dogs?number=3",
                            "protocol": "https",
                            "host": [
                                "dog-facts-api",
                                "herokuapp",
                                "com"
                            ],
                            "path": [
                                "api",
                                "v1",
                                "resources",
                                "dogs"
                            ],
                            "query": [
                                {
                                    "key": "number",
                                    "value": "3"
                                }
                            ]
                        }
                    },
                    "response": []
                }
            ]
        }
        "#;
        let config = from_string(text)?;

        assert_eq!(config.description, "Frankenline (Postman Collection)");
        Ok(())
    }
}
