use std::fs;
use std::path::Path;

use crate::config::defaults::Defaults;
use crate::config::model::Config;
use crate::error::{Error, Result};
use crate::selector::fzf_selector::FzfSelector;

pub mod config;
pub mod selector;
pub mod error;


pub fn run(config_path: &str, verbose: bool, print_config: Option<&str>, fzf_preview: Option<&str>) -> error::Result<()> {
    let config = match config::load(config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            example_config()?
        }
    };

    if let Some(format) = print_config {
        let output = match format {
            "yml" | "yaml" => serde_yaml::to_string(&config)?,
            "toml" => toml::Value::try_from(&config).unwrap().to_string(),
            _ => return Err(Error::new(format!("unknown output format \"{}\" - expected one of ( yml/yaml | toml)", format))),
        };
        eprintln!("{}", output);
        return Ok(());
    }

    if let Some(fzf_preview) = fzf_preview {
        let fzf_selector = FzfSelector::new(&config);
        let index = FzfSelector::parse_command_index(fzf_preview)?;
        let commands = &config.command_iter().collect::<Vec<_>>();
        let (config, command) = commands[index];
        print!("{}", fzf_selector.generate_fzf_preview(&config, command));
        return Ok(());
    }

    if let Some(command) = selector::select_command(&config)? {
        if verbose {
            eprintln!("selected command:\n{:?}", command);
        }
        print!("{}", command.template);
    }

    Ok(())
}

fn example_config() -> Result<Config> {
    let config_file = Path::new(Defaults::DEFAULT_TEMP_CONFIG_FILE);
    let config_file_contents = Defaults::default_config_file_contents(config_file);
    let config_file_contents = config_file_contents.trim_start();
    fs::write(&config_file, config_file_contents)?;

    eprintln!("Writing example configuration file: {}", &config_file.display());
    config::load(&config_file)
}

