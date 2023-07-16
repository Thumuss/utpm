use std::{
    env::current_dir,
    fs::{create_dir_all, read, read_dir, symlink_metadata},
};

use dirs::{config_dir, data_local_dir, data_dir};

use super::state::ErrorState;

pub fn get_config_dir() -> String {
    match config_dir() {
        Some(dir) => match dir.to_str() {
            Some(string) => String::from(string),
            None => String::from("/.config"),
        },
        None => String::from("/.config"),
    }
}

pub fn get_data_dir() -> String {
    match data_dir() {
        Some(dir) => match dir.to_str() {
            Some(string) => String::from(string),
            None => String::from("/.local/share"), //default on linux
        },
        None => String::from("/.local/share"),
    }
}

pub fn d_typst() -> String {
    get_data_dir() + "/typst"
}

pub fn d_packages() -> String {
    d_typst() + "/packages"
}

pub fn d_local() -> String {
    d_packages() + "/local"
}

pub fn global_utpm() -> String {
    get_config_dir() + "/utpm"
}

pub fn global_config() -> String {
    global_utpm() + "/.dps"
}

pub fn global_packages() -> String {
    global_utpm() + "/packages"
}

pub fn global_local_packages() -> String {
    global_packages() + "/local"
}

pub fn global_preview_packages() -> String {
    global_packages() + "/preview"
}


pub fn get_current_dir() -> Result<String, ErrorState> {
    match current_dir() {
        Ok(val) => match val.to_str() {
            Some(v) => Ok(String::from(v)),
            None => Err(ErrorState::CurrentDirectoryError(String::from(
                "there is no \".config\" directory",
            ))),
        },
        Err(val) => Err(ErrorState::CurrentDirectoryError(val.to_string())),
    }
}

pub fn current_utpm() -> Result<String, ErrorState> {
    match get_current_dir() {
        Ok(val) => Ok(val + "/.utpm"),
        Err(val) => Err(val),
    }
}

pub fn current_config() -> Result<String, ErrorState> {
    match get_current_dir() {
        Ok(val) => Ok(val + "/typst.toml"),
        Err(val) => Err(val),
    }
}

pub fn current_package() -> Result<String, ErrorState> {
    match get_current_dir() {
        Ok(val) => Ok(val + "/typst.toml"),
        Err(val) => Err(val),
    }
}

pub fn create_dir_config(path: &String) -> Result<(), ErrorState> {
    match create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(val) => Err(ErrorState::CreationDirectoryError(val.to_string())),
    }
}

pub fn check_path_dir(path: &String) -> bool {
    read_dir(path).is_ok()
}

pub fn check_path_file(path: &String) -> bool {
    read(path).is_ok()
}

#[cfg(unix)]
pub fn check_existing_symlink(path: &String) -> bool {
    let x = match symlink_metadata(path) {
        Ok(val) => val,
        Err(_) => return false,
    };
    x.file_type().is_symlink()
}
