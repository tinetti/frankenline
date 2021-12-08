use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub description: String,
    pub command: Vec<Command>,
    pub path: Option<PathBuf>,

    #[serde(skip_serializing)]
    pub import: Option<Vec<Import>>,
    pub children: Option<Vec<Config>>,

    pub fzf_command: Option<Vec<String>>,
}

impl Config {
    pub(crate) fn command_iter<'a>(&'a self) -> Box<dyn Iterator<Item=&Command> + 'a> {
        let command_iter = self.command.iter();
        match &self.children {
            None => Box::new(command_iter),
            Some(configs) => {
                let i2 = configs.iter().map(|config| config.command_iter()).flatten();
                Box::new(command_iter.chain(i2))
            }
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;


    #[test]
    fn test_command_iter() -> Result<()> {
        let config = Config {
            description: "".to_string(),
            command: vec![
                Command { name: "grandparent-c1".to_string() },
                Command { name: "grandparent-c2".to_string() },
            ],
            path: None,
            import: None,
            fzf_command: None,
            children: Some(vec![
                Config {
                    description: "".to_string(),
                    command: vec![
                        Command { name: "parent-1-c1".to_string() },
                        Command { name: "parent-1-c2".to_string() },
                    ],
                    path: None,
                    import: None,
                    fzf_command: None,
                    children: Some(vec![
                        Config {
                            description: "".to_string(),
                            command: vec![
                                Command { name: "grandchild-c1".to_string() },
                                Command { name: "grandchild-c2".to_string() },
                            ],
                            path: None,
                            import: None,
                            fzf_command: None,
                            children: None,
                        }
                    ]),
                },
                Config {
                    description: "".to_string(),
                    command: vec![
                        Command { name: "child-2-c1".to_string() },
                        Command { name: "child-2-c2".to_string() },
                    ],
                    path: None,
                    import: None,
                    fzf_command: None,
                    children: None,
                },
            ]),
        };

        for x in config.command_iter() {
            println!("{}", x.name);
        }
        let commands: Vec<&Command> = config.command_iter().collect();
        assert_eq!(commands.len(), 8);
        assert_eq!(commands[0].name, "grandparent-c1".to_string());
        assert_eq!(commands[1].name, "grandparent-c2".to_string());
        assert_eq!(commands[2].name, "parent-1-c1".to_string());
        assert_eq!(commands[3].name, "parent-1-c2".to_string());
        assert_eq!(commands[4].name, "grandchild-c1".to_string());
        assert_eq!(commands[5].name, "grandchild-c2".to_string());
        assert_eq!(commands[6].name, "child-2-c1".to_string());
        assert_eq!(commands[7].name, "child-2-c2".to_string());

        Ok(())
    }
}