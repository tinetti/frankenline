use std::io::Write;
use std::process;
use std::process::Stdio;

use crate::config::model::{Command, Config};
use crate::error::{Error, Result};
use crate::selector::CommandSelector;

pub struct FzfSelector {
    pub fzf_command: Option<Vec<String>>
}

impl FzfSelector {
    fn generate_fzf_command(self, config: &Config) -> Vec<String> {

        match self.fzf_command {
            Some(c) => c,
            None => {
                let default_layout = "default".to_string();
                let layout = &config.fzf_layout.as_ref()
                    .unwrap_or(&default_layout);

                let config_file = &config.path.as_ref().unwrap().display();
                let default_preview = format!("frankenline --config {} --fzf-preview {{}}", config_file);
                let preview = &config.fzf_preview.as_ref()
                    .unwrap_or(&default_preview);

                let default_preview_window = "down:3".to_string();
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
            },
        }
    }

    fn generate_fzf_line(index: usize, command: &&Command) -> String {
        let fzf_line = format!("{} {:30}{}\n\0", index, command.name, command.template);
        fzf_line
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

    pub fn generate_fzf_preview(config: &Config, command: &Command) -> String {
        format!(
            "config file: {}\ncommand name: {}\ncommand template: {}",
            config.path.as_ref().unwrap().display(),
            command.name,
            command.template
        )
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
            let fzf_line = Self::generate_fzf_line(i, command);
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
