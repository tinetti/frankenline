use std::fs;
use serde::{Deserialize};
use crate::config::errors::{Error, MapErrWithContext};
use crate::config::model::{Command, Config};

type Result<T> = std::result::Result<T, Error>;

pub fn from_file(path: &str) -> Result<Config> {
    let text = fs::read_to_string(path)
        .map_err_with_context(|| format!("Error loading postman file: {}", path))?;

    from_string(text.as_str())
        .map(|config| {
            Config {
                path: Some(path.to_string()),
                ..config
            }
        })
}

fn from_string(text: &str) -> Result<Config> {
    let postman: Postman = serde_json::from_str(text)
        .map_err_with_context(|| "Error parsing postman text")?;
    let config = Config::from(postman);
    Ok(config)
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
    fn test_deserialize_postman_from_string() -> Result<()> {
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

        assert_eq!(config.description, "Frankenline");
        Ok(())
    }
}
