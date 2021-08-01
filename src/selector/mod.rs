use crate::config::model::{Config, Command};
mod args_selector;

pub fn select_command<'a>(config: &'a Config, args: &Vec<String>) -> &'a Command {
    let command = args_selector::select_command_from_args(config, args);
    if let Some(args_cmd) = &command {
        return args_cmd;
    }

    panic!("no command specified!")
}
