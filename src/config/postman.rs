use serde::Deserialize;

use crate::error::{Error, Result};
use crate::config::loader::ConfigParser;
use crate::config::model::{Command, Config};

pub struct PostmanConfigParser {}

impl<S: AsRef<str>> ConfigParser<S> for PostmanConfigParser {
    fn parse(&self, json: S) -> Result<Config> {
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
    request: Request,
}

#[derive(Deserialize, Debug)]
struct Request {
    url: URL,
}

#[derive(Deserialize, Debug)]
struct URL {
    raw: String,
}


impl From<Postman> for Config {
    fn from(postman: Postman) -> Self {
        let commands = postman.item.into_iter().map(|item| {
            let template = format!("curl \"{}\"", item.request.url.raw.trim());
            Command::new(item.name, template)
        }).collect();
        let config = Config::new();
        Config {
            commands,
            description: Some(format!("{} (Postman Collection)", postman.info.name)),
            ..config
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_postman_from_string() -> Result<()> {
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
                            "raw": "  https://dog-facts-api.herokuapp.com/api/v1/resources/dogs?number=3  ",
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

        let config = PostmanConfigParser{}.parse( text)?;
        assert_eq!(config.description, "Frankenline (Postman Collection)");

        let commands = &config.commands;
        assert_eq!(commands.len(), 1);

        let command = &commands[0];
        assert_eq!(command.name, "example - get json from api");
        assert_eq!(command.template, "curl \"https://dog-facts-api.herokuapp.com/api/v1/resources/dogs?number=3\"");

        Ok(())
    }
}
