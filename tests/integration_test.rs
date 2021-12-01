#[cfg(test)]
mod tests {
    use frankenline::config;
    use frankenline::config::error::Error;

    #[test]
    fn test_from_file() -> Result<(), Error> {
        let config = config::load("tests/integration_test.toml")?;
        assert_eq!(config.description, "find and grep");
        assert_eq!(config.command.len(), 2);
        assert_eq!(config.command[0].name, "x");
        assert_eq!(config.command[1].name, "y");

        let children = config.children.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].description, "Frankenline (Postman Collection)");
        assert_eq!(children[0].command.len(), 1);
        assert_eq!(children[0].command[0].name, "example - get json from api");
        Ok(())
    }
}
