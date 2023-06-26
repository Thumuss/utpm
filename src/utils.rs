use std::{
    fs::{read_to_string, self}, process::{Command, Stdio}
};

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;


use self::{state::{ErrorState, GoodState}, paths::{get_global_config, get_global_utpm}};

pub const VERSION: &'static str = "1.0";

pub mod state;
pub mod paths;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    email: Option<String>,
    name: String,
    website: Option<String>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub link: String,
    pub authors: Option<Vec<Author>>,
    pub author: Option<Author>,
    pub main: String,
    pub auto: bool, //* ça permet juste de savoir si ça provient d'un lien github ou pas
}

impl Dependency {
    pub fn from_link(link: &String) -> Self {
        let col: Vec<&str> = link.split("/").collect();
        Self {
            name: String::from(col[4]),
            version: String::from("latest"), //TODO: Pas besoin de dire plus
            link: link.clone(),
            author: Some( Author { name: String::from(col[3]), email: None, website: None }),
            authors: Some (vec![]),
            main: String::from("./main.typ"),
            auto: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDependencies {
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

impl ListDependencies {
    pub fn new() -> Self {
        Self {
            version: VERSION.to_string(),
            dependencies: vec![],
        }
    }

    pub fn load() -> Self {
        let globpath: String = get_global_config();
        serde_json::from_str(
            read_to_string(globpath)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    pub fn write(&mut self) {
        let globpath: String = get_global_config();
        let form = serde_json::to_string(&self).unwrap();
        fs::write(globpath, form).expect("Should have write the file");
    }

    pub fn add(&mut self, link: &String) -> Result<GoodState, ErrorState> {
        let globpath: String = get_global_utpm();
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
            return Ok(GoodState::Good("Downloaded".to_string()));
        } else {
            Err(ErrorState::GitCloneError(String::from("error above ^^^^^^^^^")))
        }
        
    }

    pub fn remove(&mut self) {
        todo!()
    }
}

impl Config {
    pub fn load(path: &String) -> Self {
        serde_json::from_str(
            read_to_string(path)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    pub fn write(&mut self, path: &String) {
        let form = serde_json::to_string(&self).unwrap();
        fs::write(path, form).expect("aaa");
    }

    pub fn new(version: String, dependencies: Vec<Dependency>) -> Self {
        Self {
            version,
            dependencies,
        }
    }
}

