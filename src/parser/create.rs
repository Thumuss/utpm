use std::collections::VecDeque;

use inquire::{required, Text};

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt,
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
        let mut pkg = Package::new();

        if /*!*/(check_smt(&self.options, CLIOptions::NoInteractive)) { 
            todo!()
            /*
            pkg.name = Text::new("Name")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. my_example")
                .prompt()
                .unwrap();

            pkg.version = Text::new("Version")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. 1.0.0 (SemVer)")
                .with_default("1.0.0")
                .prompt()
                .unwrap();

            pkg.entrypoint = Text::new("Entrypoint")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. ./main.typ")
                .with_default("./main.typ")
                .prompt()
                .unwrap();

            */
        }

        TypstConfig::new(pkg).write(&typ);
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
