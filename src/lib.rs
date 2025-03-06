use std::error::Error;
use std::fs;

mod config;
pub use config::Config;

mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.get_file_path())?;

    for line in search(&config.get_query(), &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
