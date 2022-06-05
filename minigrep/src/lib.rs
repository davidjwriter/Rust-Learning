//! # Minigrep Crate
//!
//! `minigrep` is a collection of utilities to run a grep unix type function
//! from a javascript context in Rust speed.

//extern crate wasm_bindgen;
use std::env;
use std::error::Error;
use std::fs;
use std::process;
//use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn run_js(string: String, url: String) -> String {
//     let args: [String; 3] = [String::new(), string, url];
//     match run(&args) {
//         Ok(string) => string,
//         Err(_) => "No Result".to_string(),
//     }
// }

// pub fn run(args: std::env::Args) -> Result<String, Box<dyn Error>> {

//     let config = Config::new(args).unwrap_or_else(|err| {
//         eprintln!("Problem passing arguments: {}", err);
//         process::exit(1);
//     });

//     return init_search(config);
// }

pub fn init_search(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    let mut string_result = String::new();
    for line in results {
        string_result.push_str(line);
        string_result.push_str("\n");
    }
    Ok(string_result)
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();

    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    //     Ok(Config { query, filename, case_sensitive })
    // }

    // Using Iterators
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Searches the contents with the given query
///
/// # Examples
///
/// ```
/// use minigrep::search;
/// let query = "hello";
/// let contents = "hello world";
/// let results = search(&query, &contents);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    // Iterators
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents),);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        )
    }
}
