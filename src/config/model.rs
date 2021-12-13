use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub description: String,
    #[serde(alias = "command")]
    pub commands: Vec<Command>,
    #[serde(skip_serializing)]
    pub path: Option<PathBuf>,

    #[serde(skip_serializing)]
    pub children: Option<Vec<Config>>,
    #[serde(alias = "import")]
    pub imports: Option<Vec<String>>,
    #[serde(skip_serializing)]
    pub parent: Option<Box<Config>>,

    pub fzf_command: Option<Vec<String>>,
    pub fzf_line_name_width: Option<String>,
    pub fzf_layout: Option<String>,
    pub fzf_preview: Option<String>,
    pub fzf_preview_window: Option<String>,
    pub fzf_preview_description_color: Option<u8>,
    pub fzf_preview_name_color: Option<u8>,
    pub fzf_preview_path_color: Option<u8>,
    pub fzf_preview_template_color: Option<u8>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            description: "".to_string(),
            imports: None,
            path: None,
            children: None,
            parent: None,
            fzf_command: None,
            fzf_line_name_width: None,
            fzf_layout: None,
            fzf_preview: None,
            fzf_preview_window: None,
            fzf_preview_description_color: None,
            fzf_preview_name_color: None,
            fzf_preview_path_color: None,
            fzf_preview_template_color: None,
            commands: vec![]
        }
    }

    pub fn command_iter<'a>(&'a self) -> Box<dyn Iterator<Item=(&Config, &Command)> + 'a> {
        let command_iter = self.commands.iter().map(move |command| (self, command));
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
    pub template: String,
}

impl Command {
    pub(crate) fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, template: S2) -> Command {
        Command {
            name: String::from(name.as_ref()),
            template: String::from(template.as_ref()),
        }
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

    fn config(commands: Vec<Command>, children: Option<Vec<Config>>) -> Config {
        let config = Config::new();
        Config {
            commands,
            children,
            ..config
        }
    }

    fn command(name: &str) -> Command {
        Command::new(name, "")
    }

    #[test]
    fn test_command_iter() -> Result<()> {
        let config = config(
            vec![
                command("grandparent-c1"),
                command("grandparent-c2"),
            ],
            Some(vec![
                config(
                    vec![
                        command("parent-1-c1"),
                        command("parent-1-c2"),
                    ],
                    Some(vec![
                        config(
                            vec![
                                command("grandchild-c1"),
                                command("grandchild-c2"),
                            ],
                            None,
                        )
                    ]),
                ),
                config(
                    vec![
                        command("child-2-c1"),
                        command("child-2-c2"),
                    ],
                    None,
                ),
            ]),
        );

        let commands: Vec<(&Config, &Command)> = config.command_iter().collect();
        assert_eq!(commands.len(), 8);
        assert_eq!(commands[0].1.name, "grandparent-c1".to_string());
        assert_eq!(commands[1].1.name, "grandparent-c2".to_string());
        assert_eq!(commands[2].1.name, "parent-1-c1".to_string());
        assert_eq!(commands[3].1.name, "parent-1-c2".to_string());
        assert_eq!(commands[4].1.name, "grandchild-c1".to_string());
        assert_eq!(commands[5].1.name, "grandchild-c2".to_string());
        assert_eq!(commands[6].1.name, "child-2-c1".to_string());
        assert_eq!(commands[7].1.name, "child-2-c2".to_string());

        Ok(())
    }
}