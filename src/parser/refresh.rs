use std::{
    collections::VecDeque,
    process::{Command, Stdio},
};

use cmd_lib::run_cmd;

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help,
        paths::{get_current_dir, current_utpm, global_utpm},
        state::{ErrorState, GoodState},
    },
};

use super::CommandUTPM;

pub struct Refresh {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Refresh {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        match run_cmd!(curl "https://gist.githubusercontent.com/ThumusLive/a6347a1d934adaa5ff84ffed0c997baf/raw/download_official.sh" | bash) {
            Ok(_) => Ok(GoodState::Good("Success".to_string())),
            Err(val) => Err(ErrorState::UnknowError(val.to_string()))
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
