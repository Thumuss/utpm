use std::collections::VecDeque;

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

        println!("Check si ouais");

        if !check_path_dir(&globpath) {
            println!("bah j'existe pas");
            create_dir_config(&globpath)?;
        }

        if !check_path_file(&&get_global_config()) {
            ListDependencies::new().write()
        }

        let typst_config_dir = get_current_utpm()?;

        if !check_path_dir(&typst_config_dir) {
            create_dir_config(&typst_config_dir)?
        } else {
            println!("Le dossier existe déjà, skip...");
        }

        let config = get_current_config()?;

        if !check_path_file(&config) {
            println!("Création du fichier de dépendance...");
            Config::new(String::from(VERSION), vec![]).write(&config)
        } else {
            println!("Le dossier existe déjà, skip...");
        }

        Ok(GoodState::Good(String::from("Tout s'est bien passé !")))
    }

    fn help() {
        println!("Help pour new");
    }
}
