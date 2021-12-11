#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use frankenline::config;
    use frankenline::error::Result;
    use frankenline::selector::select_command;

    #[test]
    fn test_load_config() -> Result<()> {
        let yaml_config = config::load("tests/frankenline.yml")?;
        assert_eq!(yaml_config.commands.len(), 1);
        assert_eq!(yaml_config.path.unwrap(), PathBuf::from("tests/frankenline.yml"));
        let children = yaml_config.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);

        let hocon_config = children.get(0).unwrap();
        assert_eq!(hocon_config.commands.len(), 1);
        assert_eq!(hocon_config.path.as_ref().unwrap(), &PathBuf::from("tests/frankenline.hocon"));
        let children = hocon_config.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);

        let toml_config = &children.get(0).unwrap();
        assert_eq!(toml_config.commands.len(), 1);
        assert_eq!(toml_config.path.as_ref().unwrap(), &PathBuf::from("tests/frankenline.toml"));
        let children = toml_config.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);

        let postman_config = children.get(0).unwrap();
        assert_eq!(postman_config.commands.len(), 1);
        assert_eq!(postman_config.path.as_ref().unwrap(), &PathBuf::from("tests/Frankenline.postman_collection.json"));

        Ok(())
    }

    #[test]
    fn test_select_command() -> Result<()> {
        let output_filename = "/tmp/fzf-integration-test";
        let config = config::load("tests/frankenline.yml")?;
        let config = config::model::Config {
            fzf_command: Some(vec!["tee".to_string(), output_filename.to_string()]),
            ..config
        };
        let command = select_command(&config).unwrap().unwrap();

        // should get the first command since it echoes out the input and we take the first few characters of that
        assert_eq!(command.name, "edit yaml config file");

        let output_file_contents = fs::read_to_string(output_filename).unwrap();
        assert_eq!(output_file_contents,
                   r#"
0 edit yaml config file         $EDITOR $HOME/.config/frankenline.yml
1 edit hocon config file        $EDITOR $HOME/.config/frankenline.hocon
2 edit toml config file         $EDITOR $HOME/.config/frankenline.toml
3 example - get json from api   curl "https://dog-facts-api.herokuapp.com/api/v1/resources/dogs?number=3"
"#.trim_start());

        Ok(())
    }
}
