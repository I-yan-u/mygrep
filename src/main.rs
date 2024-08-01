use std::{env, process};
use mygrep::{Config, run};

fn main() {
    let opening= "--------Welcome to Minigrep--------";
    println!("{}", opening);

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Operation failed:\n{}", err);
            process::exit(1);
        });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("Case sensitive: {}\n", config.case_sensitive);

    if let Err(e) = run(config) {
        eprintln!("Unable to run search: {}", e)
    };
}