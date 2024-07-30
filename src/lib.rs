use std::error::Error;
use std::{env, fs};



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let ret = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };
    
    for ln in ret {
        println!("{}", ln);
    }
    Ok(())
}

fn search<'a >(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for lines in content.lines() {
        if lines.contains(query) {
            ret.push(lines);
        }
    }
    ret
}

fn search_insensitive<'a >(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret_in = Vec::new();

    for lines in content.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            ret_in.push(lines);
        }
    }
    ret_in
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Expected 2 arguments");
        }

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case_sensitive
        })
    }
}