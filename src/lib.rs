
use std::error::Error;
use std::fs;

mod config;
pub use config::Config;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.get_file_path())?;

    println!("With text:\n{contents}");

    Ok(())
}
