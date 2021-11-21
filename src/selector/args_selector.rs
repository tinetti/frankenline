use crate::config::model::{Command, Config};

pub(crate) fn select_command_from_args<'a>(config: &'a Config, args: &Vec<String>) -> Option<&'a Command> {
    if args.is_empty() {
        return None
    }

    println!("args: {:?}", args);

    return Some(&config.command[0]);
}
