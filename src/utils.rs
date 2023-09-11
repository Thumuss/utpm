use std::{
    fs::{self, read_to_string},
    path::Path,
};

use std::io;

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    pub compiler: Option<Version>,
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
pub struct Extra {
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct TypstConfig {
    pub package: Package,
    pub utpm: Extra,
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
        Self {
            package,
            utpm: Extra {
                version: "1".to_string(),
            },
        }
    }
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
    symlink(origin, new_path)
}

#[cfg(windows)]
pub fn symlink_all(origin: &str, new_path: &str) -> Result<(), std::io::Error> {
    use std::os::windows::fs::symlink_dir;
    symlink_dir(origin, new_path)
}
