use crate::error::Result;
use crate::config::model::{Config, Command};
use crate::selector::fzf_selector::FzfSelector;

mod fzf_selector;

pub trait CommandSelector {
    fn select_command(self, config: &Config) -> Result<Option<&Command>>;
}

pub fn select_command(config: &Config) -> Result<Option<&Command>> {
    let custom_command: Option<Vec<String>> = config.fzf_command.as_ref()
        .map(|commands| {
            commands.into_iter().map(|command| {
                format!("{}", command)
            })
                .collect()
        });

    FzfSelector {
        fzf_command: custom_command,
    }.select_command(config)
}
