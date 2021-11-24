use std::fmt::{Display, Formatter};
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

impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub enum ImportType {
    Postman
}

impl Display for ImportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
