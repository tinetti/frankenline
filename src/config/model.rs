use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub description: String,
    pub command: Vec<Command>,
    pub path: Option<PathBuf>,
    pub import: Option<Vec<Import>>,
    pub children: Option<Vec<Config>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ImportType {
    Postman
}

impl Display for ImportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
