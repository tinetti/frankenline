use std::error::Error;
use serde::{Deserialize};

fn deserialize_postman_model(text: &str) -> Result<Postman, Box<dyn Error>> {
    let postman = serde_json::from_str(text)?;
    Ok(postman)
}

#[derive(Deserialize, Debug)]
struct Postman {
    info: Info,
}

#[derive(Deserialize, Debug)]
struct Info {
    name: String,
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::*;

    #[test]
    fn test_deserialize_postman_model() -> Result<(), Box<dyn Error>> {
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
        let postman_model = deserialize_postman_model(text)?;

        assert_eq!(postman_model.info.name, "Frankenline");
        Ok(())
    }
}
