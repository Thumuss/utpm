use colored::Colorize;
use serde_json::json;
use std::fs;

use crate::utils::{
    copy_dir_all,
    paths::{check_path_dir, d_packages, get_current_dir},
    state::{Error, ErrorKind, Responses, Result},
    symlink_all, Extra, TypstConfig,
};

use super::LinkArgs;

pub fn run(cmd: &LinkArgs, path: Option<String>, res: &mut Responses) -> Result<bool> {
    let curr = path.unwrap_or(get_current_dir()?);

    let config = TypstConfig::load(&(curr.clone() + "/typst.toml"));
    let namespace = config
        .utpm
        .unwrap_or(Extra::new())
        .namespace
        .unwrap_or("local".into());

    let name = config.package.name;
    let version = config.package.version;
    let path = format!("{}/{}/{}/{}", d_packages(), namespace, name, version);
    let info = "Info:".yellow().bold();
    if check_path_dir(&path) && !cmd.force {
        return Err(Error::empty(ErrorKind::AlreadyExist(name, version, info)));
    }

    fs::create_dir_all(&path)?;

    if cmd.force {
        fs::remove_dir_all(&path)?
    }

    if cmd.no_copy {
        symlink_all(&curr, &path)?;
        let s = format!(
            "Project linked to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );
        res.push(json!({
            "message": s,
        }));
    } else {
        copy_dir_all(&curr, &path)?;
        let s = format!(
            "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );
        res.push(json!({
            "message": s,
        }));
    }
    Ok(true)
}
