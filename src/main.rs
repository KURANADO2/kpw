use kpw::Config;
use std::{env, process};

fn main() {
    let result = Config::new(env::args());
    let config = result.unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments");
        process::exit(1);
    });
    if let Err(e) = kpw::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
