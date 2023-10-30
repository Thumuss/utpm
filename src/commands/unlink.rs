use colored::Colorize;
use inquire::Confirm;
use serde_json::json;
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::{Error, ErrorKind, Responses, Result},
};

use super::UnlinkArgs;

pub fn run(cmd: &UnlinkArgs, res: &mut Responses) -> Result<bool> {
    let mut new_namespace = String::from("local");
    if let Some(nspace) = &cmd.namespace {
        new_namespace = nspace.to_owned();
    }
    if let Some(ver) = &cmd.version {
        if cmd.name.is_none() {
            return Err(Error::empty(ErrorKind::Namespace));
        }
        let ans = if !(cmd.yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!(
                        "You want to erase {}/{}",
                        cmd.name.clone().unwrap(),
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
            res.push(json!({
                "message": "Nothing to do"
            }));
            return Ok(false);
        }

        fs::remove_dir_all(
            d_packages()
                + format!(
                    "/{}/{}/{}",
                    new_namespace,
                    cmd.name.clone().unwrap(),
                    ver.to_string()
                )
                .as_str(),
        )?;
    } else if cmd.delete_namespace {
        let ans = if !(cmd.yes) {
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
            res.push(json!({
                "message": "Nothing to do"
            }));
            return Ok(false);
        }

        fs::remove_dir_all(d_packages() + format!("/{new_namespace}").as_str())?;
    } else if let Some(nm) = &cmd.name {
        let ans = if !(cmd.yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", nm).as_str())
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            res.push(json!({
                "message": "Nothing to do"
            }));
            return Ok(false);
        }

        fs::remove_dir_all(d_packages() + format!("/{}/{}", new_namespace, nm).as_str())?;
    }
    res.push(json!({
        "message": format!("{}", "Removed!".bold())
    }));
    Ok(true)
}
