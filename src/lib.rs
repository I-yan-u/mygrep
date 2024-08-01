use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("You need atleast 2 parameters to run");
        }

        let case = if args.len() == 4 && args[3] == "-i" {
            false
        } else {
            true
        };

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let ret = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    if ret.len() > 0 {
        for line in ret {
            println!("{}", line);
        };
    } else {
        println!("No results found");
    }
    
    Ok(())
}

fn search<'a >(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for ln in content.lines() {
        if ln.contains(query) {
            ret.push(ln)
        }
    }
    ret
}

fn search_case_insensitive<'a >(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for ln in content.lines() {
        if ln.to_lowercase().contains(&query.to_lowercase()) {
            ret.push(ln)
        }
    }
    ret
}