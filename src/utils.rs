use std::{
    collections::VecDeque,
    fs::{self, read_to_string},
    path::Path,
};

use std::io;

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::lexer::CLIOptions;

pub mod paths;
pub mod state;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    // Required
    pub name: String,

    pub version: Version,
    pub entrypoint: String,

    // Not required with local packages
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub description: Option<String>,

    // Not required
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub compiler: Option<String>,
    pub exclude: Option<Vec<String>>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            version: Version::new(1, 0, 0),
            entrypoint: "./main.typ".to_string(),

            authors: None,
            license: None,
            description: None,

            repository: None,
            homepage: None,
            keywords: None,
            compiler: None,
            exclude: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TypstConfig {
    pub package: Package,
}

impl TypstConfig {
    pub fn load(path: &String) -> Self {
        println!("{}", path);
        toml::from_str(
            read_to_string(path)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    pub fn write(&mut self, path: &String) {
        let form = toml::to_string_pretty(&self).unwrap();
        fs::write(path, form).expect("aaa");
    }

    pub fn new(package: Package) -> Self {
        Self { package }
    }
}

pub fn check_help(options: &VecDeque<CLIOptions>) -> bool {
    check_smt(options, CLIOptions::Help)
}

pub fn check_smt(options: &VecDeque<CLIOptions>, obj: CLIOptions) -> bool {
    options.iter().any(|val| matches!(val, a if a == &obj))
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
pub fn symlink_all(origin: &str, new_path: &str) -> Result<(), std::io::Error> {
    use std::os::unix::fs::symlink;
    return match symlink(origin, new_path) {
        Ok(_) => Ok(()),
        Err(data) => Err(data),
    };
}

#[cfg(windows)]
pub fn symlink_all(origin: &str, new_path: &str) -> Result<(), ()> {
    use std::os::windows::fs::symlink_dir;
    return match symlink_dir(origin, new_path) {
        Ok(_) => Ok(()),
        Err(data) => Err(()),
    };
}
