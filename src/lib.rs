use std::{error::Error, fs};

mod config;
pub use config::Config;

mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.get_file_path())?;

    let results = if config.get_ignore_case() {
        search_case_insensitive(&config.get_query(), &contents)
    } else {
        search(&config.get_query(), &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    let query_lowercase = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
            results.push(line);
        }
    }

    results
}
