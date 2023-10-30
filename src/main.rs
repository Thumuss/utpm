pub mod commands;
pub mod utils;

use clap::Parser;
use commands::{bulk_delete, create, install, link, list, package_path, unlink, Cli, Commands};

use utils::state::{Error, Responses};

fn main() {
    let x = Cli::parse();
    let json = x.json;
    let mut res = Responses::new(json);
    let result: Result<bool, Error> = match &x.command {
        Commands::Create(cmd) => create::run(cmd, &mut res),
        Commands::Link(cmd) => link::run(cmd, None, &mut res),
        Commands::List => list::run(&mut res),
        Commands::PackagesPath => package_path::run(&mut res),
        Commands::Unlink(cmd) => unlink::run(cmd, &mut res),
        Commands::BulkDelete(cmd) => bulk_delete::run(cmd, &mut res),
        Commands::Install(cmd) => install::run(cmd, &mut res),
    };
    match result {
        Ok(_) => {
            if json {
                print!("{}", res.json())
            }
        }
        Err(val) => {
            if json {
                eprint!("{}", val.json())
            } else {
                eprint!("{}", val.to_str())
            }
        }
    }
}
