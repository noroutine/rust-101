use std::error::Error;
use std::fs;
use std::process;

mod config;

use config::Config;
use minigrep::search;
use minigrep::search_case_insensitive;

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching {}", config.query);
    // println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path)?;
    // println!("With text:\n{contents}");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // dbg!(get_type_name(&config));
    // dbg!(&config);

    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
