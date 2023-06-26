use std::collections::VecDeque;

use crate::parser::CLIOptions;
use crate::runner::add::Install;
use crate::runner::new::New;
use crate::utils::state::{GoodState, ErrorState};

pub mod add;
pub mod new;
pub mod run;

use self::run::Run;

pub struct Runner {
    tokens: VecDeque<CLIOptions>,
}

pub trait CommandUTPM {
    fn new(tokens: VecDeque<CLIOptions>) -> Self;
    fn run (&mut self) -> Result<GoodState, ErrorState>;
    fn help();
}

impl Runner {
    pub fn new(tokens: VecDeque<CLIOptions>) -> Self {
        Self { tokens }
    }

    pub fn run(&mut self) {
        let result = match self.tokens.pop_front() {
            Some(val) => match val {
                CLIOptions::New => New::new(self.tokens.clone()).run(),
                CLIOptions::Install => Install::new(self.tokens.clone()).run(),
                CLIOptions::Run => Run::new(self.tokens.clone()).run(),
                CLIOptions::Help => {
                    Self::help();
                    Ok(GoodState::Help)
                }
                _ => todo!(),
            },
            None => {
                Self::help();
                Ok(GoodState::Help)
            },
        };

        match result {
            Ok(state) => match state {
                GoodState::Help => (),
                GoodState::Good(string) => println!("{}", string),
                GoodState::NothingToDo(string) => println!("Nothing to do! {}", string),
            },
            Err(string) => string.display(),
        }
    }

    pub fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm <commands> [options]");
        println!();
        println!("Commands: ");
        println!("  install <url|name>          Install a new package from a URL or from a NAME");
        println!("  create <name>               Create a new package to publish the project");
        println!("  delete                      Delete the actual package you are currently creating :(");
        println!("  init                        Create a new folder named \".utpm\" and create everything needed to add packages");
        println!("  compile <path>              Extension from the typst command, compile with typst a document to a pdf");
        println!("  uninstall <url|name>        Uninstall a package");
        println!();
        println!("Options: ");
        println!("  --help, -h, h               Print this message or the message assosiate to the command");
        println!();
        println!("Example:");
        println!("  utpm init");
        println!("  utpm add typst-tablex");
        println!("  echo \'#import \"/typst-tablex/tablex.typ\": tablex\' > main.typ");
        println!("  utpm run main.typ");
        println!();
        println!("Tips:");
        println!("  You can use help in front of any commands:  utpm run --help");

    }
}

fn check_help(options: &VecDeque<CLIOptions>) -> bool {
    check_smt(options, CLIOptions::Help)
}

fn check_smt(options: &VecDeque<CLIOptions>, obj: CLIOptions) -> bool {
    options.iter().any(|val| match val {
        a if a == &obj  => true,
        _ => false
    })
}

#[cfg(test)]
mod test{
    use std::collections::VecDeque;
    use super::*;
    #[test]
    fn testy(){
        let mut jpp: VecDeque<CLIOptions> = VecDeque::new();
        jpp.push_back(CLIOptions::Help);
        println!("{}", check_smt(&jpp, CLIOptions::Delete));
    }
}