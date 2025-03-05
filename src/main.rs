use std::{env, process};

use minigrep::{run, Config};

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

    if let Err(e) = run(config) {
        println!("Program error: {e}");
    }
}
