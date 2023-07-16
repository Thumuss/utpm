use std::{
    collections::VecDeque,
    fs::{self, read_to_string},
    process::{Command, Stdio}, path::Path,
};

use std::io;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::lexer::CLIOptions;

use self::{
    paths::{global_config, global_utpm},
    state::{ErrorState, GoodState},
};

pub const VERSION: &str = "1.0";

pub mod paths;
pub mod state;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    email: Option<String>,
    name: String,
    website: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub entrypoint: String,
    pub license: String,
    pub description: String,
    pub repository: Option<String>,
}

impl Dependency {
    pub fn from_link(link: &str) -> Self {
        let col: Vec<&str> = link.split('/').collect();
        Self {
            name: String::from(col[4]),
            version: String::from("latest"), //TODO: Pas besoin de dire plus
            repository: Some(link.to_owned()),
            authors: vec![],
            entrypoint: String::from("./main.typ"),
            description: String::from("Generated from ") + link,
            license: String::from("Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub package: Option<Dependency>,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDependencies {
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

impl Default for ListDependencies {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            dependencies: vec![],
        }
    }
}

impl ListDependencies {
    pub fn load() -> Self {
        let globpath: String = global_config();
        toml::from_str(
            read_to_string(globpath)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    pub fn write(&mut self) {
        let globpath: String = global_config();
        let form = toml::to_string_pretty(&self).unwrap();
        fs::write(globpath, form).expect("Should have write the file");
    }

    pub fn add(&mut self, link: &String) -> Result<GoodState, ErrorState> {
        let globpath: String = global_utpm();
        let depend = Dependency::from_link(link);

        let mut res = Command::new("git")
            .arg("clone")
            .arg(link)
            .current_dir(globpath)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Should spawn the thread");

        let status = res.wait().expect("Should run the command");

        if status.success() {
            self.dependencies.push(depend);
            self.write();
            Ok(GoodState::Good("Downloaded".to_string()))
        } else {
            Err(ErrorState::GitCloneError(String::from(
                "error above ^^^^^^^^^",
            )))
        }
    }

    pub fn remove(&mut self) {
        todo!()
    }
}

impl Config {
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

    pub fn new(
        version: String,
        dependencies: Vec<Dependency>,
        package: Option<Dependency>,
    ) -> Self {
        Self {
            package,
            version,
            dependencies,
        }
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
