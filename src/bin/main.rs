extern crate frankenline;

use clap::{App, Arg};

use frankenline::config;
use frankenline::error::Result;
use frankenline::selector;

fn main() -> Result<()> {
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
    let config = config::load(config_path)?;

    if matches.is_present(print_config_arg) {
        let config_toml = toml::Value::try_from(&config).unwrap();
        println!("\nfrankenline configuration:\n{}", config_toml)
    }

    if let Some(command) = selector::select_command(&config)? {
        println!("selected command:\n{:?}", command)
    }

    // more program logic goes here...
    Ok(())
}
