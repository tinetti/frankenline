use serde::{Deserialize};
// use toml::Deserializer;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub description: Option<String>,
}
