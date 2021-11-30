use std::fs;
use std::io;
use std::path::Path;

use crate::config::model::Config;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(toml::de::Error),
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let text = fs::read_to_string(&path)?;
    let config = from_string(text)?;
    let config = Config {
        path: Some(path.as_ref().to_path_buf()),
        ..config
    };
    Ok(config)
}

fn from_string<S: AsRef<str>>(text: S) -> Result<Config, toml::de::Error> {
    toml::from_str(text.as_ref())
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Serde(err)
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
