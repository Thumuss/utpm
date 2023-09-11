use colored::Colorize;
use inquire::Confirm;
use semver::Version;
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::{GoodResult, GoodState},
};

pub fn run(name: String, version: Option<Version>, namespace: Option<String>) -> GoodResult {
    let mut new_namespace = String::from("local");
    if let Some(nspace) = namespace {
        new_namespace = nspace;
    }
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

        fs::remove_dir_all(
            d_packages() + format!("/{}/{}/{}", new_namespace, name, ver.to_string()).as_str(),
        )?;
    } else {
        let ans: Result<bool, inquire::InquireError> =
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", name).as_str())
                .prompt();

        let bool = ans?;
        if !bool {
            return Ok(GoodState::Good("Nothing to do".to_string()));
        }

        fs::remove_dir_all(d_packages() + format!("/{}/{}", new_namespace, name).as_str())?;
    }
    println!("{}", "Removed!".bold());
    Ok(GoodState::None)
}
