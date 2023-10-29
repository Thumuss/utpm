use colored::Colorize;
use inquire::Confirm;
use semver::Version;
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::{Error, ErrorKind, Result},
};

pub fn run(
    name: &Option<String>,
    version: Option<Version>,
    namespace: Option<String>,
    yes: &bool,
    dnamespace: &bool,
) -> Result<bool> {
    let mut new_namespace = String::from("local");
    if let Some(nspace) = namespace {
        new_namespace = nspace;
    }
    if let Some(ver) = version {
        if name.is_none() {
            return Err(Error::empty(ErrorKind::Namespace));
        }
        let ans = if !(*yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!(
                        "You want to erase {}/{}",
                        name.clone().unwrap(),
                        ver.to_string()
                    )
                    .as_str(),
                )
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            println!("Nothing to do");
            return Ok(true);
        }

        fs::remove_dir_all(
            d_packages()
                + format!(
                    "/{}/{}/{}",
                    new_namespace,
                    name.clone().unwrap(),
                    ver.to_string()
                )
                .as_str(),
        )?;
    } else if *dnamespace {
        let ans = if !(*yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!("You want to erase @{new_namespace}, the namespace").as_str(),
                )
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            println!("Nothing to do");
            return Ok(true);
        }

        fs::remove_dir_all(d_packages() + format!("/{new_namespace}").as_str())?;
    } else if let Some(nm) = name {
        let ans = if !(*yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", nm).as_str())
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            println!("Nothing to do");
            return Ok(true);
        }

        fs::remove_dir_all(d_packages() + format!("/{}/{}", new_namespace, nm).as_str())?;
    }
    println!("{}", "Removed!".bold());
    Ok(true)
}
