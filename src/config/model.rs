use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub description: String,
    pub commands: Vec<Command>,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
}
