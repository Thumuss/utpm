use colored::Colorize;
use std::{collections::VecDeque, fs};

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt, copy_dir_all,
        paths::{check_path_dir, current_package, d_local, get_current_dir},
        state::{ErrorState, GoodState},
        symlink_all, TypstConfig,
    },
};

use super::CommandUTPM;

pub struct Link {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Link {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        let curr = get_current_dir()?;
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let config = TypstConfig::load(
            &(match current_package() {
                Ok(val) => val,
                Err(val) => return Err(val),
            }),
        );

        let name = config.package.name;
        let version = config.package.version;
        let path = format!("{}/{}/{}", &d_local(), name, version);
        let info = "Info:".yellow().bold();
        if check_path_dir(&path) && !check_smt(&self.options, CLIOptions::Force) {
            return Err(ErrorState::UnknowError(format!("This package ({}:{}) already exist!\n{info} Put --force to force the copy or change the version in 'typst.toml'", name, version)));
        }
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(val) => return Err(ErrorState::UnknowError(val.to_string())),
        }

        if check_smt(&self.options, CLIOptions::Force) {
            match fs::remove_dir_all(&path) {
                Ok(_) => (),
                Err(val) => return Err(ErrorState::UnknowError(val.to_string())),
            };
        }

        if check_smt(&self.options, CLIOptions::NoCopy) {
            match symlink_all(&curr, &path) {
                Ok(_) => Ok(GoodState::Good(format!(
                    "Project link to: {} \nTry importing with:\n #import \"@local/{}:{}\": *",
                    path, name, version
                ))),
                Err(val) => Err(ErrorState::UnknowError(val.to_string())),
            }
        } else {
            match copy_dir_all(get_current_dir()?, &path) {
                Ok(_) => Ok(GoodState::Good(format!(
                    "Project copied to: {}\nTry importing with:\n #import \"@local/{}:{}\": *",
                    path, name, version
                ))),
                Err(val) => Err(ErrorState::UnknowError(val.to_string())),
            }
        }
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm link");
        println!();
        println!("Description:");
        println!("  This command copy the content of the pwd and copy it in");
        println!("  the packages directory of typst");
        println!();
        println!("Options: ");
        println!("  --help,  -h                       Print this message");
        println!("  --force, -f                       Force the copy");
        println!("  --no-copy, -nc                    Use symlinks instead.");
    }
}
