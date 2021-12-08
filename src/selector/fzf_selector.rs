use std::io::Write;
use std::process;
use std::process::Stdio;

use crate::config::model::{Command, Config};
use crate::error::{Error, Result};
use crate::selector::CommandSelector;

pub struct FzfSelector {
    pub fzf_command: Option<Vec<String>>,
}

impl CommandSelector for FzfSelector {
    fn select_command(self, config: &Config) -> Result<Option<&Command>> {
        let commands: Vec<&Command> = config.command_iter().collect();

        let fzf_command = match self.fzf_command {
            Some(c) => c,
            None => vec!("fzf".to_string(),
                         "--no-multi".to_string(),
                         "--with-nth=2..".to_string()),
        };
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
        for (i, command) in commands.iter().enumerate() {
            child_stdin.write_fmt(format_args!("{} {}\n", i, command.name))
                .map_err(|err| Error::new(format!("Error writing command to fzf: {}", err)))?;
        }

        let output = child.wait_with_output()
            .map_err(|err| Error::new(format!("Error waiting for fzf command: {}", err)))?;

        let output = match output.status.code() {
            None => Err(Error::new("No command specified: fzf interrupted")),
            Some(0) => Ok(String::from_utf8_lossy(output.stdout.as_slice())),
            Some(code) => Err(Error::new(format!("No command specified: fzf returned code: {}", code))),
        }?;

        let index = output
            .split(" ")
            .next()
            .ok_or(Error::new(format!("Error: command index not found in output: {}", output)))?;

        let index = index
            .parse::<usize>()
            .map_err(|err| Error::new(format!("Error: unable to parse command index in output: '{}': {}", output, err)))?;

        let command: &Command = commands.get(index)
            .ok_or(Error::new(format!("Error: command index out of bounds ({} >= {})", index, commands.len())))?;

        Ok(Some(command))
    }
}
