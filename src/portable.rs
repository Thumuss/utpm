use std::env;

use commands::InstallArgs;
use utils::state::Responses;

#[allow(unused)]
mod commands;

#[allow(unused)]
mod utils;

/// Simple version of a portable installer
fn main() {
    let args: Vec<String> = env::args().collect();
    let force = args.contains(&"--force".to_string()) || args.contains(&"-f".to_string());
    let json = args.contains(&"--json".to_string()) || args.contains(&"-j".to_string());
    let mut res = Responses::new(json);
    let install = InstallArgs { url: None, force };
    match commands::install::run(&install, &mut res) {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => println!("Everything is good to go!"),
    }
}
