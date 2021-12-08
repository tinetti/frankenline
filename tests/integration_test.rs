#[cfg(test)]
mod tests {
    use frankenline::config;
    use frankenline::error::Result;
    use frankenline::selector::select_command;

    #[test]
    fn test_load_config() -> Result<()> {
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

    #[test]
    fn test_select_command() -> Result<()> {
        let config = config::load("tests/integration_test.toml")?;
        let config = config::model::Config {
            fzf_command: Some(vec!["tee".to_string(), "/tmp/fzf-integration-test".to_string()]),
            ..config
        };
        let command = select_command(&config).unwrap().unwrap();

        // should get the first command since it echoes out the input and we take the first few characters of that
        assert_eq!(command.name, "x");
        Ok(())
    }
}
