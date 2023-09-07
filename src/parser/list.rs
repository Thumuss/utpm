use std::collections::VecDeque;
use std::fs;

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help,
        paths::d_local,
        state::{GoodResult, GoodState},
    },
};

use super::CommandUTPM;

pub struct List {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for List {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> GoodResult {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }
        let typ = d_local();
        let dirs = fs::read_dir(&typ)?;

        println!("List: ");
        for dir_res in dirs {
            let dir = dir_res?;

            println!(" {}", dir.file_name().to_str().unwrap());

            let subdirs = fs::read_dir(dir.path())?;

            for sub_dir_res in subdirs {
                let subdir = sub_dir_res?;
                println!("  {}", subdir.file_name().to_str().unwrap());
            }
        }
        Ok(GoodState::Good(String::from("")))
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm list");
        println!();
        println!("Description:");
        println!("  List all local packages with their version");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }
}
