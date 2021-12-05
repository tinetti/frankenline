use serde::Deserialize;

use crate::config::error::Error;
use crate::config::loader::ConfigParser;
use crate::config::model::{Command, Config};

pub struct PostmanConfigParser {}

impl ConfigParser for PostmanConfigParser {
    fn parse<S: AsRef<str>>(json: S) -> Result<Config, Error> {
        let postman: Postman = serde_json::from_str(json.as_ref())?;
        let config: Config = Config::from(postman);
        Ok(config)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(err)
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
        let text: &str = r#"
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
        let config = PostmanConfigParser::parse(text)?;

        assert_eq!(config.description, "Frankenline (Postman Collection)");
        Ok(())
    }
}
