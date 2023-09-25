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

/// A modify version of the `typst.toml` adding options to utpm
#[derive(Serialize, Deserialize)]
pub struct Extra {
    /// Basic system of version (it will increased over time)
    pub version: Option<String>,
    /// The name of where you store your packages (default: local)
    pub namespace: Option<String>,

    /// List of url's for your dependencies (will be resolved with install command)
    pub dependencies: Option<Vec<String>>
}

impl Extra {
    pub fn new() -> Self {
        Self {
            version: Some("1".to_string()),
            namespace: Some("local".to_string()),
            dependencies: None
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

    pub fn new(package: Package, extra: Extra) -> Self {
        Self {
            package,
            utpm: Some(extra),
        }
    }
}

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
