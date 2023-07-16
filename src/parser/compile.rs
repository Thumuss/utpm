use std::{
    collections::VecDeque,
    process::{Command, Stdio},
};

use cmd_lib::run_cmd;

use crate::{
    lexer::CLIOptions,
    utils::{state::{GoodState, ErrorState}, paths::{current_utpm, get_current_dir}, check_help},
};

use super::CommandUTPM;

pub struct Compile {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Compile {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let token = match self.options.pop_front() {
            Some(val) => match val {
                CLIOptions::Token(string) => string,
                _ => return Err(ErrorState::UnexpectedTokenError(String::from("Expected to find a file"))),
            },
            None => return Ok(GoodState::Help)
        };

        let cdir = get_current_dir()?;
        let cutpm = current_utpm()?;

        match run_cmd!(TYPST_ROOT=$cutpm typst -c $cdir/$token) {
            Ok(_) => Ok(GoodState::Good("Success".to_string())),
            Err(val) =>  Err(ErrorState::UnknowError(val.to_string()))
        }
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm compile <FILE>");
        println!();
        println!("Description:");
        println!("  This command is an extension of the command `typst compile`. It calls this command with an");
        println!("  env variable. You can mimic this command by doing this :");
        println!("  `TYPST_ROOT=\"$ABSOLUTE_PATH_TO_UTPM_FOLDER\" typst compile <FILE>`");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }

   
}
