use minigrep;
use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Minigrep error: {}", err);
        process::exit(1);
    });

    println!("Searching for : {}", config.query);
    println!("In file       : {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Minigrep Error: {}", e);
        process::exit(1);
    }
}
