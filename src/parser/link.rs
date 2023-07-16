use std::{
    collections::VecDeque,
    fs,
    os::unix::fs::symlink,
    process::{Command, Stdio},
};

use cmd_lib::run_cmd;

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help,
        paths::{
            check_existing_symlink, check_path_dir, current_package, current_utpm, d_local,
            d_packages, get_current_dir, global_local_packages, global_utpm,
        },
        state::{ErrorState, GoodState},
        Config,
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
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }
        let packages = &global_local_packages();
        if !check_path_dir(packages) {
            fs::create_dir_all(packages);
        }

        if !check_path_dir(&d_packages().to_string()) {
            fs::create_dir_all(packages);
        }

        let config = Config::load(
            &(match current_package() {
                Ok(val) => val,
                Err(val) => return Err(val),
            }),
        );

        if let Some(package) = config.package {
            let x = package.name;
            let y = package.version;
            let name = format!("{}/{}-{}", packages, x, y);
            fs::create_dir_all(&name);
            match run_cmd!(
                cp -r ./ $name/;
            ) {
                Err(val) => println!("{}", val.to_string()),
                _ => (),
            };
            if !check_path_dir(&d_packages()) {
                fs::create_dir_all(&d_packages());
            }
            if !check_existing_symlink(&d_local()) {
                match symlink(global_local_packages(), d_local()) {
                    Ok(_) => (),
                    Err(val) => {
                        println!("{}", global_local_packages());
                        return Err(ErrorState::SymlinkUnixError(val.to_string()));
                    }
                };
            }
        } else {
            return Err(ErrorState::UnknowError("aaa".to_string()));
        }
        Ok(GoodState::Good("good".to_string()))
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
