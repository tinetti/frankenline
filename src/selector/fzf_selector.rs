use std::collections::HashMap;
use std::io::Write;
use std::process;
use std::process::Stdio;

use ansi_term::Color::Fixed;
use strfmt::Format;
use crate::config::defaults::Defaults;

use crate::config::model::{Command, Config};
use crate::error::{Error, Result};
use crate::selector::CommandSelector;

pub struct FzfSelector {
    pub fzf_command: Option<Vec<String>>,
    pub fzf_line_name_width: u8,
    pub fzf_preview_description_color: u8,
    pub fzf_preview_name_color: u8,
    pub fzf_preview_path_color: u8,
    pub fzf_preview_template_color: u8,
}

impl FzfSelector {
    pub fn new(config: &Config) -> FzfSelector {
        let name_width = &config.fzf_line_name_width;
        let name_width = name_width.as_ref()
            .map(|text| text.parse::<u8>())
            .unwrap_or(Ok(Defaults::DEFAULT_FZF_LINE_NAME_WIDTH))
            .unwrap_or_else(|err| {
                eprintln!("error parsing fzf_line_name_width: {}", err);
                Defaults::DEFAULT_FZF_LINE_NAME_WIDTH
            });

        FzfSelector {
            fzf_command: None,
            fzf_line_name_width: name_width,

            fzf_preview_description_color: config.fzf_preview_description_color
                .unwrap_or(Defaults::DEFAULT_FZF_PREVIEW_DESCRIPTION_COLOR),

            fzf_preview_name_color: config.fzf_preview_name_color
                .unwrap_or(Defaults::DEFAULT_FZF_PREVIEW_NAME_COLOR),

            fzf_preview_path_color: config.fzf_preview_path_color
                .unwrap_or(Defaults::DEFAULT_FZF_PREVIEW_PATH_COLOR),

            fzf_preview_template_color: config.fzf_preview_template_color
                .unwrap_or(Defaults::DEFAULT_FZF_PREVIEW_TEMPLATE_COLOR),
        }
    }

    fn generate_fzf_line(&self, index: usize, command: &Command) -> String {
        let fmt = format!("{{index}} {{name:<{name_width}}} {{template}}\0", name_width = self.fzf_line_name_width);
        let index = format!("{}", index.to_string());
        let name = format!("{}", Fixed(self.fzf_preview_name_color).paint(&command.name));
        let template = format!("{}", Fixed(self.fzf_preview_template_color).paint(&command.template));
        let mut vars = HashMap::new();
        vars.insert("index".to_string(), &index);
        vars.insert("name".to_string(), &name);
        vars.insert("template".to_string(), &template);
        fmt.format(&vars).unwrap_or(fmt)
    }

    pub fn generate_fzf_preview(&self, config: &Config, command: &Command) -> String {
        let path = format!("{}", config.path.as_ref().unwrap().display());
        format!(
            "[{path}]\n{description}\n\n{name}\n{template}",
            path = Fixed(self.fzf_preview_path_color).paint(path),
            description = Fixed(self.fzf_preview_description_color).paint(&config.description),
            name = Fixed(self.fzf_preview_name_color).paint(&command.name),
            template = Fixed(self.fzf_preview_template_color).paint(&command.template),
        )
    }

    pub fn parse_command_index<S: AsRef<str>>(fzf_line: S) -> Result<usize> {
        let fzf_line = fzf_line.as_ref();
        let index = fzf_line
            .split(" ")
            .next()
            .ok_or(Error::new(format!("Error: command index not found in fzf line: {}", fzf_line)))?;

        let index = index
            .parse::<usize>()
            .map_err(|err| Error::new(format!("Error: unable to parse command index in fzf line: '{}': {}", fzf_line, err)))?;

        Ok(index)
    }

    fn generate_fzf_command(&self, config: &Config) -> Vec<String> {
        let c = &self.fzf_command;
        match c {
            Some(c) => c.to_owned(),
            None => {
                let default_layout = "default".to_string();
                let layout = &config.fzf_layout.as_ref()
                    .unwrap_or(&default_layout);

                let config_file = &config.path.as_ref().unwrap().display();
                let default_preview = format!("frankenline --config {config} --fzf-preview {{}}", config = config_file);
                let preview = &config.fzf_preview.as_ref()
                    .unwrap_or(&default_preview);

                let default_preview_window = "up:5".to_string();
                let preview_window = &config.fzf_preview_window.as_ref()
                    .unwrap_or(&default_preview_window);

                vec!("fzf".to_string(),
                     "--ansi".to_string(),
                     "--with-nth=2..".to_string(),
                     "--read0".to_string(),
                     format!("--layout={}", layout),
                     format!("--preview={}", preview),
                     format!("--preview-window={}", preview_window),
                )
            }
        }
    }
}

impl CommandSelector for FzfSelector {
    fn select_command(self, config: &Config) -> Result<Option<&Command>> {
        let commands: Vec<(&Config, &Command)> = config.command_iter().collect();
        let fzf_command = self.generate_fzf_command(config);
        let mut fzf_command = fzf_command.into_iter();

        let fzf_exe = fzf_command.next()
            .ok_or(Error::new("Error: no fzf command specified"))?;

        let mut child = process::Command::new(fzf_exe)
            .args(fzf_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| Error::new(format!("Error spawning fzf command: {}", err)))?;

        let child_stdin = child.stdin.as_mut().unwrap();
        for (i, (_config, command)) in commands.iter().enumerate() {
            let fzf_line = self.generate_fzf_line(i, command);
            let fzf_line = fzf_line.as_bytes();
            child_stdin.write(fzf_line)
                .map_err(|err| Error::new(format!("Error writing command to fzf: {}", err)))?;
        }

        let output = child.wait_with_output()
            .map_err(|err| Error::new(format!("Error waiting for fzf command: {}", err)))?;

        let output = match output.status.code() {
            None => Err(Error::new("No command specified: fzf interrupted")),
            Some(0) => Ok(String::from_utf8_lossy(output.stdout.as_slice())),
            Some(code) => Err(Error::new(format!("No command specified: fzf returned code: {}", code))),
        }?;

        let index = Self::parse_command_index(output.as_ref())?;
        let (_config, command) = commands.get(index)
            .ok_or(Error::new(format!("Error: command index out of bounds ({} >= {})", index, commands.len())))?;

        Ok(Some(command))
    }
}
