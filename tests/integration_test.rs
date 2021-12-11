#[cfg(test)]
mod tests {
    use std::fs;
    use frankenline::config;
    use frankenline::error::Result;
    use frankenline::selector::select_command;

    #[test]
    fn test_load_config() -> Result<()> {
        let config = config::load("tests/integration_test.toml")?;
        assert_eq!(config.description, "find and grep");
        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.commands[0].name, "x");
        assert_eq!(config.commands[1].name, "y");

        let children = config.children.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].description, "Frankenline (Postman Collection)");
        assert_eq!(children[0].commands.len(), 1);
        assert_eq!(children[0].commands[0].name, "example - get json from api");
        Ok(())
    }

    #[test]
    fn test_select_command() -> Result<()> {
        let output_filename = "/tmp/fzf-integration-test";
        let config = config::load("tests/integration_test.toml")?;
        let config = config::model::Config {
            fzf_command: Some(vec!["tee".to_string(), output_filename.to_string()]),
            ..config
        };
        let command = select_command(&config).unwrap().unwrap();

        // should get the first command since it echoes out the input and we take the first few characters of that
        assert_eq!(command.name, "x");

        let output_file_contents = fs::read_to_string(output_filename).unwrap();
        assert_eq!(output_file_contents,
                   "\
0 x                             echo \"x\"
\01 y                             echo \"y\"
\02 example - get json from api   curl \"https://dog-facts-api.herokuapp.com/api/v1/resources/dogs?number=3\"
\0");

        Ok(())
    }
}
