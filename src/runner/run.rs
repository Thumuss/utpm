use std::{
    collections::VecDeque,
    process::{Command, Stdio},
};

use crate::{
    parser::CLIOptions,
    utils::{state::{GoodState, ErrorState}, paths::{get_current_utpm, get_current_dir}},
};

use super::{CommandUTPM, check_help};

pub struct Run {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Run {
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

        let mut res = Command::new("typst")
            .env("TYPST_ROOT", get_current_utpm()?.as_str())
            .arg("c")
            .arg(token)
            .current_dir(get_current_dir()?)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Should spawn the thread");


        let status = res.wait().expect("Should run the command");

        if status.success() {
            Ok(GoodState::Good("Success".to_string()))
        } else {
            Err(ErrorState::TypstCompileError(String::from("error above ^^^^^^^^^^^^")))
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
