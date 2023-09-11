use colored::Colorize;
use inquire::Confirm;
use semver::Version;
use std::fs;

use crate::utils::{
    paths::d_local,
    state::{GoodResult, GoodState},
};

pub(crate) fn unlink(name: String, version: Option<Version>) -> GoodResult {
    if let Some(ver) = version {
        let ans: Result<bool, inquire::InquireError> =
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!("You want to erase {}/{}", name, ver.to_string()).as_str(),
                )
                .prompt();

        let bool = ans?;
        if !bool {
            return Ok(GoodState::Good("Nothing to do".to_string()));
        }

        fs::remove_dir_all(d_local() + format!("/{}/{}", name, ver.to_string()).as_str())?;
    } else {
        let ans: Result<bool, inquire::InquireError> =
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", name).as_str())
                .prompt();

        let bool = ans?;
        if !bool {
            return Ok(GoodState::Good("Nothing to do".to_string()));
        }

        fs::remove_dir_all(d_local() + format!("/{}", name).as_str())?;
    }
    println!("{}", "Removed!".bold());
    Ok(GoodState::None)
}
