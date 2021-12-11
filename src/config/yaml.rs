use crate::config::loader::ConfigParser;
use crate::config::model::Config;
use crate::error::{Error, Result};

pub struct YamlConfigParser {}

impl<S: AsRef<str>> ConfigParser<S> for YamlConfigParser {
    fn parse(&self, yaml: S) -> Result<Config> {
        let config = serde_yaml::from_str(yaml.as_ref())?;
        Ok(config)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::new(err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<()> {
        let text = r"
            description: find and grep
            commands:
            - name: x
              template: echo x
            - name: y
              template: echo x
        ";
        let config = YamlConfigParser {}.parse(text)?;

        assert_eq!(config.description, "find and grep");
        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.commands[0].name, "x");
        assert_eq!(config.commands[1].name, "y");
        Ok(())
    }
}
