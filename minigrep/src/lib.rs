use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
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
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    if res.is_empty() {
        Err(format!("Target file does not contain '{}'", query))
    } else {
        Ok(res)
    }
}