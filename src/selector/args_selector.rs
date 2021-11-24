use crate::config::model::{Command, Config};

pub(crate) fn select_command_from_args<'a>(config: &'a Config, args: &Vec<String>) -> Option<&'a Command> {
    println!("args: {:?}", args);

    if args.is_empty() {
        return None
    }

    return Some(&config.command[0]);
}
