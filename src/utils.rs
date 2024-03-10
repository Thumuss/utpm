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
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
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

    pub categories: Option<Vec<Categorie>>,
    pub disciplines: Option<Vec<Discipline>>,

    /// A minimum version of the compiler (for typst)
    pub compiler: Option<Version>,
    /// A list of excludes files
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Discipline {

}


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Categorie {
    Components,
    Visualization,
    Model,
    Layout,
    Text,
    Languages,
    Scripting,
    Integration,
    Utility,
    Fun,

    Book,
    Report,
    Paper,
    Thesis,
    Poster,
    Flyer,
    Presentation,
    CV,
    Office,
}

//todo: move it to a new file (and everything else)
impl Categorie {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Components => "components",
            Self::Visualization => "visualization",
            Self::Model => "model",
            Self::Layout => "layout",
            Self::Text => "text",
            Self::Languages => "languages",
            Self::Scripting => "scripting",
            Self::Integration => "integration",
            Self::Utility => "utility",
            Self::Fun => "fun",

            Self::Book => "book",
            Self::Report => "report",
            Self::Paper => "paper",
            Self::Thesis => "thesis",
            Self::Poster => "poster",
            Self::Flyer => "flyer",
            Self::Presentation => "presentation",
            Self::CV => "cv",
            Self::Office => "office",
        }
    }

    fn from_str(string: &str) -> Result<Self, String> { //todo: Change result
        match string {
           "components," => Ok(Self::Components),
           "visualization," =>  Ok(Self::Visualization),
           "model," =>  Ok(Self::Model),
           "layout," =>  Ok(Self::Layout),
           "text," =>  Ok(Self::Text),
           "languages," =>  Ok(Self::Languages),
           "scripting," =>  Ok(Self::Scripting),
           "integration," =>  Ok(Self::Integration),
           "utility," =>  Ok(Self::Utility),
           "fun," =>  Ok(Self::Fun),

           "book," =>  Ok(Self::Book),
           "report," =>  Ok(Self::Report),
           "paper," =>  Ok(Self::Paper),
           "thesis," =>  Ok(Self::Thesis),
           "poster," =>  Ok(Self::Poster),
           "flyer," =>  Ok(Self::Flyer),
           "presentation," =>  Ok(Self::Presentation),
           "cv," =>  Ok(Self::CV),
           "office," =>  Ok(Self::Office),
           _ => Err("unknown type".into()),
        }
    }
}



/// Default implementation of a package
impl Package {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            version: Version::new(1, 0, 0),
            entrypoint: "main.typ".to_string(),

            authors: Some(vec![]),
            license: None,
            description: None,

            repository: None,
            homepage: None,
            keywords: None,
            categories: Some(vec![]),
            disciplines: Some(vec![]),
            compiler: None,
            exclude: None,
        }
    }
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
}

impl Extra {
    pub fn new() -> Self {
        Self {
            version: Some("2".to_string()),
            namespace: Some("local".to_string()),
            dependencies: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tools {
    /// An extra for utpm (v3 only)
    pub utpm: Option<Extra>,
}

impl Tools {
    pub fn new(utpm: Option<Extra>) -> Self {
        Self { utpm }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub path: String,
    pub entrypoint: String,
    pub thumbnail: String,
}

/// The file `typst.toml` itself
#[derive(Serialize, Deserialize)]
pub struct TypstConfig {
    /// Base of typst package system
    pub package: Package,
    /// An extra for utpm (v2 only)
    pub utpm: Option<Extra>,

    /// Base on the new structure of typst package
    pub tool: Option<Tools>,

    pub template: Option<Template>,
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
    pub fn new(package: Package, extra: Option<Extra>, template: Option<Template>) -> Self {
        Self {
            package,
            utpm: None, // We allow only readable versions.
            template,
            tool: Some(Tools::new(extra)),
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
