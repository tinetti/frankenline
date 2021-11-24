extern crate frankenline;

use std::error::Error;

use clap::{App, Arg};
use frankenline::config;


fn main() -> Result<(), Box<dyn Error>> {
    let config_arg = "config";
    let print_config_arg = "print-commands";

    let matches = App::new("Frankenline")
        .version("1.0")
        .author("John Tinetti <john@tinetti.net>")
        .about("Cheetsheet Builder")
        .arg(Arg::with_name(config_arg)
            .short("c")
            .long("config")
            .value_name("FILE")
            .default_value("./frankenline.toml")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name(print_config_arg)
            .short("p")
            .long("print-config")
            .help("Prints out the resolved configuration"))
        .get_matches();

    let config_path = matches.value_of(config_arg).unwrap();
    let config = match config::loader::load(config_path) {
        Ok(c) => c,
        Err(e) => panic!("Execution failed!\n{}", e)
    };

    if matches.is_present(print_config_arg) {
        let config_toml = toml::Value::try_from(&config).unwrap();
        println!("\nfrankenline.toml:\n{}", config_toml)
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
