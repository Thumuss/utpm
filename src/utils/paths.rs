use std::{env::current_dir, fs::{create_dir_all, read_dir, read, symlink_metadata}};

use dirs::config_dir;

use super::state::ErrorState;


pub fn get_global_dir() -> String {
    match config_dir() {
        Some(dir) => match dir.to_str() {
            Some(string) => String::from(string),
            None => String::from("/.config"),
        },
        None => String::from("/.config"),
    }
}

pub fn get_global_utpm() -> String {
    get_global_dir() + "/utpm"
}

pub fn get_global_config() -> String {
    get_global_utpm() + "/.dps"
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

pub fn get_current_utpm() -> Result<String, ErrorState> {
    match get_current_dir() {
        Ok(val) => Ok(val + "/.utpm"),
        Err(val) => Err(val),
    }
}

pub fn get_current_config() -> Result<String, ErrorState> {
    match get_current_utpm() {
        Ok(val) => Ok(val + "/.config"),
        Err(val) => Err(val),
    }
}

pub fn get_current_package() -> Result<String, ErrorState> {
    match get_current_utpm() {
        Ok(val) => Ok(val + "/.package"),
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
    match read_dir(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn check_path_file(path: &String) -> bool {
    match read(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn check_existing_symlink(path: &String) -> bool {
    let x = match symlink_metadata(path) {
        Ok(val) => val,
        Err(_) => return false
    };
    x.file_type().is_symlink()
}
