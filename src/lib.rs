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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowercase))
        .collect()
}
