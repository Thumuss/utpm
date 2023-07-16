use std::collections::VecDeque;

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help,
        paths::{check_path_file, get_current_dir},
        state::{ErrorState, GoodState},
        Package, TypstConfig,
    },
};

use super::CommandUTPM;

pub struct Create {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Create {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }
        let typ = get_current_dir()? + "/typst.toml";
        if check_path_file(&typ) {
            return Ok(GoodState::NothingToDo);
        }

        TypstConfig::new(Package::new()).write(&typ);
        Ok(GoodState::Good("File created!".to_string()))
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm create");
        println!();
        println!("Description:");
        println!("  This create a dummy typst.toml, you need to fill it manually for now (wip)");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }
}
