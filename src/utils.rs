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
/// Represent a package from the official `typst.toml`
/// See https://github.com/typst/packages
pub struct Package {
    // Required

    /// The name of the package
    pub name: String,
    /// The version (using semver)
    pub version: Version,
    /// Where is your main file
    pub entrypoint: String,

    // Not required with local packages

    /// The authors of the package
    pub authors: Option<Vec<String>>,
    /// The license
    pub license: Option<String>,
    /// A little description for your users
    pub description: Option<String>,

    // Not required

    /// A link to your repository
    pub repository: Option<String>,
    /// The link to your website
    pub homepage: Option<String>,
    /// A list of keywords to research your package
    pub keywords: Option<Vec<String>>,
    /// A minimum version of the compiler (for typst)
    pub compiler: Option<Version>,
    /// A list of excludes files
    pub exclude: Option<Vec<String>>,
}

/// Default implementation of a package
impl Package {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            version: Version::new(1, 0, 0),
            entrypoint: "main.typ".to_string(),

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
#[derive(Serialize, Deserialize, Clone)]
pub enum ProjectType {
    Template,
    Package
}

/// A modify version of the `typst.toml` adding options to utpm
#[derive(Serialize, Deserialize, Clone)]
pub struct Extra {
    /// Basic system of version (it will increased over time to keep track of what change or not)
    pub version: Option<String>,
    /// The name of where you store your packages (default: local)
    pub namespace: Option<String>,

    /// List of url's for your dependencies (will be resolved with install command)
    pub dependencies: Option<Vec<String>>,

    /// A quick implementation of a type system of projects
    pub types: Option<ProjectType>
}

impl Extra {
    pub fn new() -> Self {
        Self {
            version: Some("2".to_string()),
            namespace: Some("local".to_string()),
            dependencies: None,
            types: Some(ProjectType::Package)
        }
    }
}

/// The file `typst.toml` itself
#[derive(Serialize, Deserialize)]
pub struct TypstConfig {
    /// Base of typst package system
    pub package: Package,
    /// An extra for utpm
    pub utpm: Option<Extra>,

}

impl TypstConfig {
    /// Load the configuration from a file
    pub fn load(path: &String) -> Self {
        toml::from_str(
            read_to_string(path)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    /// Write a file
    pub fn write(&mut self, path: &String) {
        let form = toml::to_string_pretty(&self).unwrap();
        fs::write(path, form).expect("aaa");
    }

    /// Create a typstConfig
    pub fn new(package: Package, extra: Extra) -> Self {
        Self {
            package,
            utpm: Some(extra),
        }
    }
}

/// Copy all subdirectories from a point to an other
/// From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
/// Edited to prepare a portable version
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() && entry.file_name() != "utpmp" && entry.file_name() != "install" {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Implementing a symlink function for all platform (unix version)
#[cfg(unix)]
pub fn symlink_all(origin: &str, new_path: &str) -> Result<(), std::io::Error> {
    use std::os::unix::fs::symlink;
    symlink(origin, new_path)
}

/// Implementing a symlink function for all platform (windows version)
#[cfg(windows)]
pub fn symlink_all(origin: &str, new_path: &str) -> Result<(), std::io::Error> {
    use std::os::windows::fs::symlink_dir;
    symlink_dir(origin, new_path)
}
