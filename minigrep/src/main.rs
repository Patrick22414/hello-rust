use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for : {}", config.query);
    println!("In file       : {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("--- Something went wrong when reading the file!");

    println!("With text     :");
    for line in contents.split("\n") {
        println!("\t{}", line);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
