use std::{env, fs, path::Path};

use crate::utils::{
    paths::{
        check_path_dir, check_path_file, d_packages, datalocalutpm, get_current_dir, get_ssh_dir,
    },
    state::{Error, ErrorKind, Result, Responses},
    TypstConfig,
};
use colored::Colorize;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};

use super::link;

pub fn run(force: bool, url: Option<&String>, mut res: Responses) -> Result<Responses> {
    let path = format!("{}/tmp", datalocalutpm());
    if check_path_dir(&path) {
        fs::remove_dir_all(path)?;
    }
    init(force, url, 0, res)
}

pub fn init(force: bool, url: Option<&String>, i: usize,  mut res: Responses) -> Result<Responses> {
    let path = if url.is_none() {
        get_current_dir()?
    } else {
        format!("{}/tmp/{}", datalocalutpm(), i)
    };

    if let Some(x) = url {
        fs::create_dir_all(&path)?;
        let sshpath = get_ssh_dir()?;
        let ed = sshpath.clone() + "/id_ed25519";
        let rsa = sshpath + "/id_rsa";
        let val = if check_path_file(&ed) { ed } else { rsa };
        if x.starts_with("git") {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_, username_from_url, _| {
                match Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                    Ok(cred) => Ok(cred),
                    Err(_) => Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
                        None,
                        Path::new(&val),
                        Some(
                            env::var("UTPM_PASSPHRASE")
                                .unwrap_or(String::new())
                                .as_str(),
                        ),
                    ),
                }
            });

            let mut fo = FetchOptions::new();
            fo.remote_callbacks(callbacks);

            let mut builder = RepoBuilder::new();
            builder.fetch_options(fo);
            builder.clone(&x, Path::new(&path))?;
        } else {
            Repository::clone(&x, &path)?;
        }
    };

    let typstfile = path.clone() + "/typst.toml";
    if !check_path_file(&typstfile) {
        return Err(Error::empty(ErrorKind::ConfigFile));
    }

    let file = TypstConfig::load(&typstfile);
    let utpm = file.utpm;
    let namespace = utpm
        .clone()
        .unwrap_or(crate::utils::Extra {
            version: None,
            namespace: Some("local".to_string()),
            dependencies: None,
            types: Some(crate::utils::ProjectType::Package),
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
        return Ok(true);
    }

    println!("{}", format!("Installing {}...", file.package.name).bold());
    if let Some(fl) = utpm {
        if let Some(vec_depend) = fl.dependencies {
            let mut y = 0;
            let vec_of_dependencies = vec_depend
                .iter()
                .map(|a| -> Result<bool> {
                    y += 1;
                    init(force, Some(a), i * vec_depend.len() + y) // idk
                })
                .collect::<Vec<Result<bool>>>();

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
        println!(
            "{}",
            "* Installation complete! If you want to use it as a lib, just do a `utpm link`!"
                .bold()
        )
    }

    Ok(true)
}
