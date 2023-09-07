use colored::Colorize;
use std::{collections::VecDeque, fs};

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt, copy_dir_all,
        paths::{check_path_dir, current_package, d_local, get_current_dir},
        state::{ErrorState, GoodResult, GoodState},
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

    fn run(&mut self) -> GoodResult {
        let curr = get_current_dir()?;
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let config = TypstConfig::load(&current_package()?);

        let name = config.package.name;
        let version = config.package.version;
        let path = format!("{}/{}/{}", &d_local(), name, version);
        let info = "Info:".yellow().bold();
        if check_path_dir(&path) && !check_smt(&self.options, CLIOptions::Force) {
            return Err(ErrorState::UnknowError(format!("This package ({}:{}) already exist!\n{info} Put --force to force the copy or change the version in 'typst.toml'", name, version)));
        }

        fs::create_dir_all(&path)?;

        if check_smt(&self.options, CLIOptions::Force) {
            fs::remove_dir_all(&path)?
        }

        if check_smt(&self.options, CLIOptions::NoCopy) {
            symlink_all(&curr, &path)?;
            Ok(GoodState::Good(format!(
                "Project link to: {} \nTry importing with:\n #import \"@local/{}:{}\": *",
                path, name, version
            )))
        } else {
            copy_dir_all(get_current_dir()?, &path)?;
            Ok(GoodState::Good(format!(
                "Project licopiednk to: {} \nTry importing with:\n #import \"@local/{}:{}\": *",
                path, name, version
            )))
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
