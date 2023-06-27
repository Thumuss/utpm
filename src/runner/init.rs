use std::collections::VecDeque;
use colored::Colorize;
use crate::{
    parser::CLIOptions,
    utils::{Config, ListDependencies, VERSION, state::{GoodState, ErrorState}, paths::{get_global_utpm, check_path_dir, create_dir_config, check_path_file, get_global_config, get_current_utpm, get_current_config}}, runner::check_help,
};

use super::CommandUTPM;


pub struct New {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for New {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let globpath = get_global_utpm();

        print!("◼ Checking if the global utpm folder exist...");

        if !check_path_dir(&globpath) {
            println!("{}", " ✖".red());
            print!("  - Creating the config dir...");
            create_dir_config(&globpath)?;
            println!("{}", " ✔".green());
        } else {
            println!("{}", " ✔".green());

        }

        print!("◼ Checking if the global utpm config file exist...");


        if !check_path_file(&&get_global_config()) {
            println!("{}", " ✖".red());
            print!("  - Creating the config file...");
            ListDependencies::new().write();
            println!("{}", " ✔".green());
        } else {
            println!("{}", " ✔".green());
        }

        print!("◼ Checking if there is a working directory...");

        let typst_config_dir = get_current_utpm()?;

        println!("{}", " ✔".green());
        print!("◼ Checking if the current .utpm dir exist...");

        if !check_path_dir(&typst_config_dir) {
            println!("{}", " ✖".red());
            print!("  - Creating the dir...");
            create_dir_config(&typst_config_dir)?;
            println!("{}", " ✔".green());
        } else {
            println!("{}", " ✔".green());
        }

        let config = get_current_config()?;

        print!("◼ Checking if the current .utpm config file exist...");

        if !check_path_file(&config) {
            println!("{}", " ✖".red());
            print!("  - Creating the file...");
            Config::new(String::from(VERSION), vec![]).write(&config);
            println!("{}", " ✔".green());
        } else {
            println!("{}", " ✔".green());
        }

        Ok(GoodState::Good(String::from("All good!")))
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm init");
        println!();
        println!("Description:");
        println!("  This command creates multiple directories. First it creates a directory at ~/.config/utpm");
        println!("  and then it creates a directory at $PWD/.utpm. It add a file called \".config\" in it and");
        println!("  it create a file in the config dir named \".dps\" (for dependencies)");
        println!("  All theses files are written in JSON. Please do not edit them.");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }
}
