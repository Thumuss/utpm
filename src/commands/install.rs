use std::fs;

use crate::utils::{
    paths::{check_path_file, datalocalutpm, get_current_dir, check_path_dir},
    state::{ErrorState, GoodResult, GoodState},
    TypstConfig,
};
use colored::Colorize;
use git2::Repository;

use super::link;

pub fn run(force: bool, url: Option<&String>) -> GoodResult {
    fs::remove_dir_all(&format!("{}/tmp", datalocalutpm()))?;
    init(force, url, 0)
}

pub fn init(force: bool, url: Option<&String>, i: usize) -> GoodResult {
    let path = if url.is_none() {
        get_current_dir()?
    } else {
        format!("{}/tmp/{}", datalocalutpm(), i)
    };

    fs::create_dir_all(&path)?;
    
    if let Some(x) = url {
        Repository::clone(&x, &path)?;
    };
    let typstfile = path.clone() + "/typst.toml";
    if !check_path_file(&typstfile) {
        return Err(ErrorState::UnknowError("Pas de typsttoml fdp".to_string()));
    }

    let file = TypstConfig::load(&typstfile);

    if check_path_dir(&path) {
        println!("{}", format!("~ {}:{}", file.package.name, file.package.version).bright_black());
        return Ok(GoodState::None);
    }
    
    println!("Installing {}...", file.package.name);
    if let Some(fl) = file.utpm {
        if let Some(vec_depend) = fl.dependencies {
            let mut y = 0;
            let vec_of_dependencies = vec_depend
                .iter()
                .map(|a| -> GoodResult {
                    y += 1;
                    init(force, Some(a), i * vec_depend.len() + y) // idk
                })
                .collect::<Vec<GoodResult>>();

            for result_dependencies in vec_of_dependencies {
                result_dependencies?;
            }
        }
    }

    if !url.is_none() {
        fs::remove_dir_all(&path)?;
        link::run(force, false, Some(path.clone()))?;
        println!("{}", format!("+ {}:{}", file.package.name, file.package.version).bright_green());
    }
    Ok(GoodState::None)
}
