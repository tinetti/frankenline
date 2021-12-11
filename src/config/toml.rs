use crate::error::{Error, Result};
use crate::config::loader::ConfigParser;
use crate::config::model::Config;

pub struct TomlConfigParser {}

impl<S: AsRef<str>> ConfigParser<S> for TomlConfigParser {
    fn parse(&self, toml: S) -> Result<Config> {
        let config = toml::from_str(toml.as_ref())?;
        Ok(config)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::new(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<()> {
        let text = r"
            description = 'find and grep'

            [[command]]
            name = 'x'
            template = 'echo x'

            [[command]]
            name = 'y'
            template = 'echo y'
        ";
        let config = TomlConfigParser{}.parse(text)?;

        assert_eq!(config.description, "find and grep");
        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.commands[0].name, "x");
        assert_eq!(config.commands[1].name, "y");
        Ok(())
    }
}
