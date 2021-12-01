use crate::config::error::Error;
use crate::config::loader::ConfigParser;
use crate::config::model::Config;

pub struct TomlConfigParser {}

impl ConfigParser for TomlConfigParser {
    fn parse<S: AsRef<str>>(toml: S) -> Result<Config, Error> {
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
    fn test_from_string() -> Result<(), Error> {
        let text = r#"
            description = 'find and grep'

            [[command]]
            name = 'x'

            [[command]]
            name = 'y'
        "#;
        let config = from_string(text)?;

        assert_eq!(config.description, "find and grep");
        assert_eq!(config.command.len(), 2);
        assert_eq!(config.command[0].name, "x");
        assert_eq!(config.command[1].name, "y");
        Ok(())
    }
}
