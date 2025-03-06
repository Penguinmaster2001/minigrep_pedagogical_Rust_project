use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config;
    match Config::build(&args) {
        Ok(ok) => config = ok,
        Err(err) => {
            eprintln!("Error parsing args: {err}");
            println!("Usage: {} <query> <file path>", args[0]);
            process::exit(1)
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Program error: {e}");
        process::exit(2)
    }
}
