use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let config;
    match Config::build(env::args()) {
        Ok(ok) => config = ok,
        Err(err) => {
            eprintln!("Error parsing args: {err}");
            println!(
                "Usage: {} <query> <file path>",
                env::args().next().unwrap_or(String::from("minigrep"))
            );
            process::exit(1)
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Program error: {e}");
        process::exit(2)
    }
}
