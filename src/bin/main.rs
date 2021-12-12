extern crate frankenline;

use clap::{App, Arg};

use frankenline::{default_config_file_path, run};

fn main() {
    let config_arg = "config";
    let print_config_arg = "print-config";
    let fzf_preview_arg = "fzf-preview";
    let verbose_arg = "debug";
    let default_config_arg = default_config_file_path();
    let default_config_arg = format!("{}", default_config_arg.display());

    let matches = App::new("Frankenline")
        .version("1.0")
        .author("John Tinetti <john@tinetti.net>")
        .about("Cheetsheet Builder")
        .arg(Arg::with_name(config_arg)
            .short("c")
            .long(config_arg)
            .help("Sets a custom config file")
            .takes_value(true)
            .value_name("FILE")
            .default_value(&default_config_arg)
        )
        .arg(Arg::with_name(print_config_arg)
            .short("p")
            .long(print_config_arg)
            .help("Prints out the resolved configuration (one of yml/yaml | toml)")
            .takes_value(true)
            .value_name("FORMAT")
        )
        .arg(Arg::with_name(fzf_preview_arg)
            .short("f")
            .long(fzf_preview_arg)
            .help("Prints out the command summary for the fzf preview window")
            .takes_value(true)
            .value_name("FZF_OUTPUT")
        )
        .arg(Arg::with_name(verbose_arg)
            .short("v")
            .long(verbose_arg)
            .help("Prints verbose information")
        )
        .get_matches();

    let config_path = matches.value_of(config_arg).unwrap();
    let verbose = matches.is_present(verbose_arg);
    let print_config = matches.value_of(print_config_arg);
    let fzf_preview = matches.value_of(fzf_preview_arg);

    run(config_path, verbose, print_config, fzf_preview).unwrap_or_else(|err| {
        eprintln!("{}", err.message);
    })
}
