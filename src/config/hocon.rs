use crate::error::{Error, Result};
use crate::config::loader::ConfigParser;
use crate::config::model::Config;

pub struct HoconConfigParser {}

impl<S: AsRef<str>> ConfigParser<S> for HoconConfigParser {
    fn parse(&self, hocon: S) -> Result<Config> {
        let config = hocon::de::from_str(hocon.as_ref())?;
        Ok(config)
    }
}

impl From<hocon::Error> for Error {
    fn from(err: hocon::Error) -> Self {
        Error::new(err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<()> {
        let text = r"{
            description: find and grep
            commands: [
                {
                    name: x
                    template: echo x
                },
                {
                    name: y
                    template: echo x
                }
            ]
        }";
        let config = HoconConfigParser {}.parse(text)?;

        assert_eq!(config.description, "find and grep");
        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.commands[0].name, "x");
        assert_eq!(config.commands[1].name, "y");
        Ok(())
    }
}
