use colored::Colorize;
use std::fs;

use crate::utils::{
    copy_dir_all,
    paths::{check_path_dir, d_packages, get_current_dir},
    state::{Error, ErrorKind, Result, Responses},
    symlink_all, Extra, TypstConfig,
};

pub fn run(force: bool, no_copy: bool, path: Option<String>, mut res: Responses) -> Result<Responses> {
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
    if check_path_dir(&path) && !force {
        return Err(Error::empty(ErrorKind::AlreadyExist(name, version, info)));
    }

    fs::create_dir_all(&path)?;

    if force {
        fs::remove_dir_all(&path)?
    }

    if no_copy {
        symlink_all(&curr, &path)?;
        println!(
            "Project linked to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );
        Ok(true)
    } else {
        copy_dir_all(&curr, &path)?;
        println!(
            "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );
        Ok(true)
    }
}
