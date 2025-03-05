mod config;

use config::Config;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config;
    match Config::build(&args) {
        Ok(ok) => config = ok,
        Err(err) => {
            println!("Error parsing args: {err}");
            println!("Usage: {} <query> <file path>", args[0]);
            process::exit(0)
        }
    };

    println!("Searching for {}", config.get_query());
    println!("In file {}", config.get_file_path());

    let contents = fs::read_to_string(config.get_file_path()).expect("Could not read file");

    println!("With text:\n{contents}");
}
