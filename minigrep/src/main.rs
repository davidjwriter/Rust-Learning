use std::env;
use std::process;

pub use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem passing arguments: {}", err);
    //     process::exit(1);
    // });

    // use iterators

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    let response = match minigrep::init_search(config) {
        Ok(string) => string,
        Err(_) => "error".to_string(),
    };

    println!("{}", response);
}
