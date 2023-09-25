use colored::Colorize;
use inquire::Confirm;
use semver::Version;
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::{GoodResult, GoodState},
};

pub fn run(
    name: String,
    version: Option<Version>,
    namespace: Option<String>,
    yes: &bool,
) -> GoodResult {
    let mut new_namespace = String::from("local");
    if let Some(nspace) = namespace {
        new_namespace = nspace;
    }
    if let Some(ver) = version {
        let ans = if !(*yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!("You want to erase {}/{}", name, ver.to_string()).as_str(),
                )
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            return Ok(GoodState::Message("Nothing to do".to_string()));
        }

        fs::remove_dir_all(
            d_packages() + format!("/{}/{}/{}", new_namespace, name, ver.to_string()).as_str(),
        )?;
    } else {
        let ans = if !(*yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", name).as_str())
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            return Ok(GoodState::Message("Nothing to do".to_string()));
        }

        fs::remove_dir_all(d_packages() + format!("/{}/{}", new_namespace, name).as_str())?;
    }
    println!("{}", "Removed!".bold());
    Ok(GoodState::None)
}
