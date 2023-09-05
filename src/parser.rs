use std::collections::VecDeque;

use crate::lexer::CLIOptions;
use crate::utils::state::{GoodState, ErrorState};


pub mod link;
pub mod unlink;
pub mod create;
pub mod list;

use self::create::Create;
use self::link::Link;
use self::list::List;

pub struct Parser {
    tokens: VecDeque<CLIOptions>,
}

pub trait CommandUTPM {
    fn new(tokens: VecDeque<CLIOptions>) -> Self ;
    fn run (&mut self) -> Result<GoodState, ErrorState>;
    fn help();
}

impl Parser {
    pub fn new(tokens: VecDeque<CLIOptions>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) {
        let result = match self.tokens.pop_front() {
            Some(val) => match val {
                CLIOptions::Link => Link::new(self.tokens.clone()).run(),
                CLIOptions::Create => Create::new(self.tokens.clone()).run(),
                CLIOptions::List => List::new(self.tokens.clone()).run(),
                CLIOptions::Help => {
                    Self::help();
                    Ok(GoodState::Help)
                }
                _ => Err(ErrorState::UnknowError(String::from("Unknown command!"))),
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
                GoodState::NothingToDo => println!("Nothing to do!"),
                GoodState::None => (),
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
        println!("  create                      Create a new package to link the project");
        println!("  link                        Copy the folder into your share folder to make a new package (need a typst.toml file)");
        println!("  list                        List all packages from the local directory");
        //println!("  unlink                      Remove the package of typst");
        println!();
        println!("Options: ");
        println!("  --help, -h, h               Print this message or the message assosiate to the command");
        println!();
        println!("Example:");
        println!("  utpm create");
        println!("  echo \"#let x = 12\" > main.typ");
        println!("  utpm link");
        println!("  echo \'#import \"@local/example:1.0.0\": x; #x\' > main.typ");
        println!("  typst c main.typ");
        println!();
        println!("Tips:");
        println!("  You can use help in front of any commands:  utpm run --help");

    }
}

