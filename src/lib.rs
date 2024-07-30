use std::error::Error;
use std::fs;



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let ret = search(&config.query, &content);
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

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Expected 2 arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}