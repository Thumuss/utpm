use std::{env, path::Path};

use crate::utils::{
    paths::{check_path_file, current_package, get_ssh_dir},
    state::{Error, ErrorKind, ResponseKind, Responses, Result},
    TypstConfig,
};
use git2::{Cred, Remote, RemoteCallbacks, Repository};
use owo_colors::OwoColorize;

use super::InfoArgs;

pub fn run(cmd: &InfoArgs, res: &mut Responses) -> Result<bool> {
    let path = cmd.url.clone().unwrap_or(current_package()?);
    if !check_path_file(&path) {
        return Err(Error::empty(ErrorKind::ConfigFile));
    }

    let config = TypstConfig::load(&path);
    if let Some(utpm) = config.utpm {
        for e in utpm.dependencies.unwrap_or(vec![]) {
            if e.starts_with(".") {
                match Repository::open(&e) {
                    Ok(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "✔️".green().bold(),
                        e
                    ))),
                    Err(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "❌".red().bold(),
                        e
                    ))),
                };
            } else if (&e).starts_with("http") {
                let mut remote = Remote::create_detached(e.clone())?;

                match remote.connect(git2::Direction::Fetch) {
                    Ok(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "✔️".green().bold(),
                        e
                    ))),
                    Err(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "❌".red().bold(),
                        e
                    ))),
                };
            } else if e.starts_with("git") {
                let sshpath = get_ssh_dir()?;
                let ed = sshpath.clone() + "/id_ed25519";
                let rsa = sshpath + "/id_rsa";
                let val = if check_path_file(&ed) { ed } else { rsa };
                let mut remote = Remote::create_detached(e.clone())?;
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
                match remote.connect_auth(git2::Direction::Fetch, Some(callbacks), None) {
                    Ok(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "✔️".green().bold(),
                        e
                    ))),
                    Err(_) => res.push(ResponseKind::Message(format!(
                        "{}: {}",
                        "❌".red().bold(),
                        e
                    ))),
                };
            }
        }
    }

    Ok(true)
}
