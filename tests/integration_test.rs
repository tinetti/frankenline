#[cfg(test)]
mod tests {
    use std::error::Error;
    use frankenline::config;

    #[test]
    fn test_from_file() -> Result<(), Box<dyn Error>> {
        let config = config::loader::load("tests/integration_test.toml")?;
        assert_eq!(config.description, "find and grep");
        assert_eq!(config.command.len(), 3);
        assert_eq!(config.command[0].name, "x");
        assert_eq!(config.command[1].name, "y");
        assert_eq!(config.command[2].name, "example - get json from api");
        Ok(())
    }
}
