use std::collections::VecDeque;

use inquire::{required, Confirm, Text};

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt,
        paths::{check_path_file, get_current_dir},
        state::{ErrorState, GoodState},
        Package, TypstConfig,
    },
};

use super::CommandUTPM;

pub struct Create {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Create {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }
        let typ = get_current_dir()? + "/typst.toml";
        let force = check_smt(&self.options, CLIOptions::Force);
        if check_path_file(&typ) && !force {
            return Ok(GoodState::NothingToDo);
        }

        if force {
            println!("WARNING: --force is a dangerous flag, use it cautiously")
        }

        let mut pkg = Package::new();

        if !(check_smt(&self.options, CLIOptions::NoInteractive)) {
            pkg.name = Text::new("Name: ")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. my_example")
                .prompt()
                .unwrap();

            pkg.version = Text::new("Version: ")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. 1.0.0 (SemVer)")
                .with_default("1.0.0")
                .prompt()
                .unwrap();

            pkg.entrypoint = Text::new("Entrypoint: ")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. ./main.typ")
                .with_default("./main.typ")
                .prompt()
                .unwrap();

            if Confirm::new("Are you going to publish it?")
                .with_default(false)
                .with_help_message("More questions will come if yes")
                .prompt()
                .unwrap()
            {
                pkg.authors = Some(
                    Text::new("Authors: ")
                        .with_help_message("e.g. Thumus,Somebody,Somebody Else")
                        .prompt()
                        .unwrap()
                        .split(",")
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>(),
                );

                pkg.license = Some(
                    Text::new("license: ")
                        .with_help_message("e.g. MIT")
                        .prompt()
                        .unwrap(),
                );

                pkg.description = Some(
                    Text::new("description: ")
                        .with_help_message("e.g. A package")
                        .prompt()
                        .unwrap(),
                );

                if Confirm::new("Do you want more options ?")
                    .with_default(false)
                    .with_help_message(
                        "Options are : repository, homepage, keywords, compiler, exclude",
                    )
                    .prompt()
                    .unwrap()
                {
                    pkg.repository = Some(
                        Text::new("URL of the repository: ")
                            .with_help_message("e.g. https://github.com/ThumusLive/unofficial-typst-package-manager")
                            .prompt()
                            .unwrap(),
                    );
                    pkg.homepage = Some(
                        Text::new("Homepage: ")
                            .with_help_message("e.g. anything")
                            .prompt()
                            .unwrap(),
                    );
                    pkg.keywords = Some(
                        Text::new("Authors: ")
                            .with_help_message("e.g. Typst,keyword")
                            .prompt()
                            .unwrap()
                            .split(",")
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>(),
                    );
                    pkg.compiler = Some(
                        Text::new("Compiler version required: ")
                            .with_help_message("e.g. 0.7.0")
                            .prompt()
                            .unwrap(),
                    );
                    pkg.exclude = Some(
                        Text::new("Exclude: ")
                            .with_help_message("e.g. backup/mypassword.txt,.env")
                            .prompt()
                            .unwrap()
                            .split(",")
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>(),
                    );
                }
            }
        }

        TypstConfig::new(pkg).write(&typ);
        Ok(GoodState::Good("File created!".to_string()))
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm create");
        println!();
        println!("Description:");
        println!("  This create a dummy typst.toml, you need to fill it manually for now (wip)");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
        println!("  --force, -f                             Force the creation of the file (warning)");
    }
}
