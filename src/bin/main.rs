extern crate frankenline;

use clap::{App, Arg};

use frankenline::config;
use frankenline::error::Result;
use frankenline::selector;
use frankenline::selector::fzf_selector::FzfSelector;

fn main() -> Result<()> {
    let config_arg = "config";
    let print_config_arg = "print-config";
    let fzf_preview_arg = "fzf-preview";
    let verbose_arg = "debug";

    let matches = App::new("Frankenline")
        .version("1.0")
        .author("John Tinetti <john@tinetti.net>")
        .about("Cheetsheet Builder")
        .arg(Arg::with_name(config_arg)
            .short("c")
            .long(config_arg)
            .value_name("FILE")
            .default_value("./frankenline.toml")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name(print_config_arg)
            .short("p")
            .long(print_config_arg)
            .help("Prints out the resolved configuration"))
        .arg(Arg::with_name(fzf_preview_arg)
            .short("f")
            .long(fzf_preview_arg)
            .help("Prints out the command summary for the fzf preview window")
            .takes_value(true))
        .arg(Arg::with_name(verbose_arg)
            .short("v")
            .long(verbose_arg)
            .help("Prints verbose information"))
        .get_matches();

    let config_path = matches.value_of(config_arg).unwrap();
    let config = config::load(config_path)?;

    let verbose = matches.is_present(verbose_arg);

    if matches.is_present(print_config_arg) {
        let config_toml = toml::Value::try_from(&config).unwrap();
        eprintln!("\nfrankenline configuration:\n{}", config_toml);
        return Ok(());
    }

    if let Some(fzf_preview) = matches.value_of(fzf_preview_arg) {
        let index = FzfSelector::parse_command_index(fzf_preview)?;
        // let index = fzf_preview.parse::<usize>()
        //     .map_err(|err| Error::new(format!("Error: unable to parse command index in '{}': {}", fzf_preview, err)))?;
        let commands = &config.command_iter().collect::<Vec<_>>();
        let (config, command) = commands[index];
        print!("{}", FzfSelector::generate_fzf_preview(config, command));
        return Ok(());
    }

    if let Some(command) = selector::select_command(&config)? {
        if verbose {
            eprintln!("selected command:\n{:?}", command);
        }
        print!("{}", command.template);
    }

    // more program logic goes here...
    Ok(())
}
