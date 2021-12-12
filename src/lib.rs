use std::fs;
use std::path::{Path, PathBuf};

use crate::config::model::Config;
use crate::error::{Error, Result};
use crate::selector::fzf_selector::FzfSelector;

pub mod config;
pub mod selector;
pub mod error;

pub fn default_config_file() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();
    let file = format!("{}/.config/frankenline.yml", home.display());
    PathBuf::from(file)
}

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
        let index = FzfSelector::parse_command_index(fzf_preview)?;
        let commands = &config.command_iter().collect::<Vec<_>>();
        let (config, command) = commands[index];
        print!("{}", FzfSelector::generate_fzf_preview(config, command));
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
    let config_file = Path::new("/tmp/frankenline.example.yml");
    let config_file_contents = format!("
description: Welcome to Frankenline!  Here are some sample commands to get you started :)

commands:
- name: print this config file
  template: frankenline --print-config yaml
- name: copy this config file to your home directory
  template: cp {path} ~/.config/frankenline.example.yml
- name: edit frankenline config file
  template: eval ${{EDITOR:-vi}} {path}

imports: ~
fzf_command: ~
fzf_layout: ~
fzf_preview: ~
fzf_preview_window: ~
",
        path = &config_file.display()
    );
    let config_file_contents = config_file_contents.trim_start();
    fs::write(&config_file, config_file_contents)?;

    eprintln!("Writing example configuration file: {}", &config_file.display());
    config::load(&config_file)
}

