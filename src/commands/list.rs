use std::fs;
use colored::Colorize;

use crate::utils::{
    paths::d_local,
    state::{GoodResult, GoodState},
};

pub fn list() -> GoodResult {
    let typ = d_local();
    let dirs = fs::read_dir(&typ)?;

    println!("{}", "Tree listing of your packages\n".bold());
    println!("{}", "@local: ".bright_green());
    for dir_res in dirs {
        let dir = dir_res?;

        println!("  {}:", dir.file_name().to_str().unwrap().green().bold());

        let subdirs = fs::read_dir(dir.path())?;

        for sub_dir_res in subdirs {
            let subdir = sub_dir_res?;
            println!("    - {}", subdir.file_name().to_str().unwrap().green());
        }
    }
    Ok(GoodState::Good(String::from("")))
}
