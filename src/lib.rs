
use std::error::Error;
use std::fs;

use config::Config;

pub mod config;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.get_file_path())?;

    println!("With text:\n{contents}");

    Ok(())
}
