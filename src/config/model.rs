use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub description: String,
    pub command: Vec<Command>,
    pub import: Option<Vec<Import>>,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Import {
    #[serde(rename = "type")]
    pub import_type: ImportType,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub enum ImportType {
    Postman
}
