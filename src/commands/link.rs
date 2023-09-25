use colored::Colorize;
use std::fs;

use crate::utils::{
    copy_dir_all,
    paths::{check_path_dir, get_current_dir, d_packages},
    state::{ErrorState, GoodResult, GoodState},
    symlink_all, TypstConfig, Extra,
};

pub fn run(force: bool, no_copy: bool, path: Option<String>) -> GoodResult {
    let curr = path.unwrap_or(get_current_dir()?);

    let config = TypstConfig::load(&(curr.clone() + "/typst.toml"));
    let namespace = config.utpm.unwrap_or(Extra::new()).namespace.unwrap_or("local".to_string());

    let name = config.package.name;
    let version = config.package.version;
    let path = format!("{}/{}/{}/{}", d_packages(), namespace, name, version);
    let info = "Info:".yellow().bold();
    if check_path_dir(&path) && !force {
        return Err(ErrorState::UnknowError(format!("This package ({}:{}) already exist!\n{info} Put --force to force the copy or change the version in 'typst.toml'", name, version)));
    }

    fs::create_dir_all(&path)?;

    if force {
        fs::remove_dir_all(&path)?
    }


    if no_copy {
        symlink_all(&curr, &path)?;
        Ok(GoodState::Message(format!(
            "Project link to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace , name, version
        )))
    } else {
        copy_dir_all(&curr, &path)?;
        Ok(GoodState::Message(format!(
            "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        )))
    }
}
