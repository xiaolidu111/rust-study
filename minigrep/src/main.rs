use minigrep::Config;
use std::{env, process};
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::parse_config(env::args()).unwrap_or_else(|err| {
        eprintln!("The error is {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error is {}", e);
        process::exit(1);
    }
}
