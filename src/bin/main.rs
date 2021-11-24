extern crate frankenline;

use std::error::Error;

use clap::{App, Arg};
use frankenline::config;


fn main() -> Result<(), Box<dyn Error>> {
    let config_arg = "config";
    let print_commands_arg = "print-commands";

    let matches = App::new("Frankenline")
        .version("1.0")
        .author("John Tinetti <john@tinetti.net>")
        .about("Cheetsheet Builder")
        .arg(Arg::with_name(config_arg)
            .short("c")
            .long("config")
            .value_name("FILE")
            .default_value("~/.config/frankenline.toml")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name(print_commands_arg)
            .short("p")
            .help("Prints out the resolved commands"))
        .get_matches();

    let config_path = matches.value_of(config_arg).unwrap();
    let config = match config::loader::load(config_path) {
        Ok(c) => c,
        Err(e) => panic!("Execution failed!\n{}", e)
    };

    if matches.is_present(print_commands_arg) {
        for command in config.command {
            println!("{:?}", command);
        }
    }

    // more program logic goes here...
    Ok(())
}


// fn main() -> Result<(), Box<dyn Error>> {
//     println!("Hello, world!");
//     let config = frankenline::config::loader::load("frankenline.toml")?;
//     let mut args: Vec<String> = env::args().collect();
//
//     // trim off the first arg since it's just the frankenline command itself
//     args.drain(0..1);
//
//     let command = frankenline::selector::select_command(&config, &args);
//     println!("{}", command.name);
//     Ok(())
// }
