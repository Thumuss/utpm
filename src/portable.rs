use std::env;

#[allow(unused)]
mod commands;

#[allow(unused)]
mod utils;

/// Simple version of a portable installer
fn main() {
    let args: Vec<String> = env::args().collect();
    let force = args.contains(&"--force".to_string()) || args.contains(&"-f".to_string());
    match commands::install::run(force, None) {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => println!("Everything is good to go!"),
    }
}
