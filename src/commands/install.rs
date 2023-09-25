use std::fs;

use crate::utils::{
    paths::{check_path_dir, check_path_file, d_packages, datalocalutpm, get_current_dir},
    state::{ErrorState, GoodResult, GoodState},
    TypstConfig,
};
use colored::Colorize;
use git2::Repository;

use super::link;

pub fn run(force: bool, url: Option<&String>) -> GoodResult {
    let path = format!("{}/tmp", datalocalutpm());
    if check_path_dir(&path) {
        fs::remove_dir_all(path)?;
    }
    init(force, url, 0)
}

pub fn init(force: bool, url: Option<&String>, i: usize) -> GoodResult {
    let path = if url.is_none() {
        get_current_dir()?
    } else {
        format!("{}/tmp/{}", datalocalutpm(), i)
    };

    if let Some(x) = url {
        fs::create_dir_all(&path)?;
        Repository::clone(&x, &path)?;
    };

    let typstfile = path.clone() + "/typst.toml";
    if !check_path_file(&typstfile) {
        return Err(ErrorState::UnknowError("Pas de typsttoml fdp".to_string()));
    }

    let file = TypstConfig::load(&typstfile);
    let utpm = file.utpm;
    let namespace = utpm
        .clone()
        .unwrap_or(crate::utils::Extra {
            version: None,
            namespace: Some("local".to_string()),
            dependencies: None,
        })
        .namespace
        .unwrap_or("local".into());

    if check_path_dir(&format!(
        "{}/{}/{}/{}",
        d_packages(),
        namespace,
        &file.package.name,
        &file.package.version
    )) {
        println!(
            "{}",
            format!("~ {}:{}", file.package.name, file.package.version).bright_black()
        );
        return Ok(GoodState::None);
    }


    println!("{}", format!("Installing {}...", file.package.name).bold());
    if let Some(fl) = utpm {
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
        link::run(force, false, Some(path.clone()))?;
        fs::remove_dir_all(&path)?;
        println!(
            "{}",
            format!("+ {}:{}", file.package.name, file.package.version).bright_green()
        );
    } else {
        println!("{}", "* Installation complete! If you want to use it as a lib, just do a `utpm link`!".bright_green())
    }
    
    Ok(GoodState::None)
}
