extern crate frankenline;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    let config = frankenline::config::loader::load_from_file("frankenline.toml")?;
    println!("name: {:?}", config);
    Ok(())
}
