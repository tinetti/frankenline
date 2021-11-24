use std::fs;

use crate::config::errors::{Error, MapErrWithContext};
use crate::config::model::Config;

type Result<T> = std::result::Result<T, Error>;


pub fn from_file(path: &str) -> Result<Config> {
    let text = fs::read_to_string(path)
        .map_err_with_context(|| format!("Error loading from file: {}", path))?;
    from_string(text.as_str())
}

fn from_string(text: &str) -> Result<Config> {
    let config = toml::from_str(text)
        .map_err_with_context(|| "Error parsing toml")?;
    Ok(config)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<()> {
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
