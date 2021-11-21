use std::error::Error;
use std::fs;

use super::model::Config;

pub fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    from_string(text.as_str())
}

fn from_string(text: &str) -> Result<Config, Box<dyn Error>> {
    let config = toml::from_str(text)?;
    Ok(config)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<(), Box<dyn Error>> {
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
