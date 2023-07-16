use std::{
    collections::VecDeque,
    fs::{self, read_to_string},
    path::Path,
};

use std::io;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::lexer::CLIOptions;

pub mod paths;
pub mod state;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub entrypoint: String,
    pub license: String,
    pub description: String,
    pub repository: Option<String>,
}


impl Package {
    pub fn new() -> Self {
        Self {
            name: "example".to_string(),
            version: "1.0.0".to_string(), 
            repository: Some("example".to_string()),
            authors: vec!["Thumus".to_string()],
            entrypoint: String::from("./main.typ"),
            description: String::from("An example"),
            license: String::from("MIT"),
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
