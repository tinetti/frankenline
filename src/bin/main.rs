extern crate frankenline;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let config = frankenline::config::loader::load("frankenline.toml")?;
    let mut args: Vec<String> = env::args().collect();

    // trim off the first arg since it's just the frankenline command itself
    args.drain(0..1);

    let command = frankenline::selector::select_command(&config, &args);
    println!("{}", command.name);
    Ok(())
}
