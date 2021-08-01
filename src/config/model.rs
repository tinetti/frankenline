use serde::{Deserialize};
// use toml::Deserializer;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub commands: Vec<Command>,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub command: String,
    // pub description: Option<String>,
}

