mod config;

use config::Config;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.get_query());
    println!("In file {}", config.get_file_path());

    let contents = fs::read_to_string(config.get_file_path()).expect("Could not read file");

    println!("With text:\n{contents}");
}
