use colored::Colorize;
use std::fs;

use crate::utils::{
    copy_dir_all,
    paths::{check_path_dir, current_package, get_current_dir, d_packages},
    state::{ErrorState, GoodResult, GoodState},
    symlink_all, TypstConfig,
};

pub fn run(force: bool, no_copy: bool) -> GoodResult {
    let curr = get_current_dir()?;

    let config = TypstConfig::load(&current_package()?);

    let name = config.package.name;
    let version = config.package.version;
    let path = format!("{}/{}/{}/{}", d_packages(), config.utpm.namespace, name, version);
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
        Ok(GoodState::Good(format!(
            "Project link to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, config.utpm.namespace , name, version
        )))
    } else {
        copy_dir_all(get_current_dir()?, &path)?;
        Ok(GoodState::Good(format!(
            "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, config.utpm.namespace, name, version
        )))
    }
}
