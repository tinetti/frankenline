use std::error::Error;
use std::fs;
use std::path::Path;

use super::model::Config;

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    load_from_string(text.as_str())
}

fn load_from_string(text: &str) -> Result<Config, Box<dyn Error>> {
    let config = toml::from_str(text)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() -> Result<(), Box<dyn Error>> {
        let text = r#"
            name = 'find and grep'

            [args]
            github = 'xxxxxxxxxxxxxxxxx'
            travis = 'yyyyyyyyyyyyyyyyy'
        "#;
        let config = load_from_string(text)?;

        assert_eq!(config.name, "find and grep");
        Ok(())
    }
}
