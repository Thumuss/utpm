use std::collections::VecDeque;

use crate::lexer::CLIOptions;
use crate::parser::install::Install;
use crate::parser::init::New;
use crate::utils::state::{GoodState, ErrorState};

pub mod install;
pub mod init;
pub mod compile;
pub mod refresh;
pub mod link;
pub mod unlink;

use self::compile::Compile;
use self::link::Link;
use self::refresh::Refresh;

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
                CLIOptions::Init => New::new(self.tokens.clone()).run(),
                CLIOptions::Install => Install::new(self.tokens.clone()).run(),
                CLIOptions::Refresh => Refresh::new(self.tokens.clone()).run(),
                CLIOptions::Compile => Compile::new(self.tokens.clone()).run(),
                CLIOptions::Link => Link::new(self.tokens.clone()).run(),
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

