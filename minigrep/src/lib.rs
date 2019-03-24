use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // consume the first arg (minigrep.exe)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot parse 'query'"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot parse 'filename'"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text     :");
    for line in contents.lines() {
        println!("\t{}", line);
    }

    let result = search(&config.query, &contents)?;

    println!("Found '{}' in lines:", config.query);
    for res_line in result {
        println!("\t{}", res_line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, String> {
    let res: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    if res.is_empty() {
        Err(format!("Target file does not contain '{}'", query))
    } else {
        Ok(res)
    }
}
