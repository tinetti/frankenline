use std::path::{Path, PathBuf};

pub struct Defaults {}

impl Defaults {
    pub const DEFAULT_TEMP_CONFIG_FILE: &'static str = "/tmp/frankenline.example.yml";

    pub const DEFAULT_FZF_LINE_NAME_WIDTH: u8 = 75;
    pub const DEFAULT_FZF_PREVIEW_DESCRIPTION_COLOR: u8 = 1;
    pub const DEFAULT_FZF_PREVIEW_NAME_COLOR: u8 = 2;
    pub const DEFAULT_FZF_PREVIEW_PATH_COLOR: u8 = 3;
    pub const DEFAULT_FZF_PREVIEW_TEMPLATE_COLOR: u8 = 4;

    pub fn default_config_file_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_default();
        let file = format!("{}/.config/frankenline.yml", home.display());
        PathBuf::from(file)
    }

    pub fn default_config_file_contents(config_file: &Path) -> String {
        format!("
description: Welcome to Frankenline!  Here are some sample commands to get you started :)

commands:
- name: print this config file
  template: frankenline --print-config yaml

- name: copy this config file to your home directory
  template: cp {path} ~/.config/frankenline.example.yml

- name: edit frankenline config file
  template: eval ${{EDITOR:-vi}} {path}

imports: []
",
                path = &config_file.display()
        )
    }
}