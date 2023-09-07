use colored::Colorize;
use std::collections::VecDeque;

use inquire::{required, validator::Validation, Select, Text};
use semver::Version;

use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt,
        paths::{check_path_file, get_current_dir},
        state::{ErrorState, GoodState, GoodResult},
        Package, TypstConfig,
    },
};

use super::CommandUTPM;

static TYPES: [&str; 3] = ["Local", "Public", "Public with more options"];

pub struct Create {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Create {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> GoodResult {
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
            println!(
                "{} {}",
                "WARNING:".bold().yellow(),
                "--force is a dangerous flag, use it cautiously".bold()
            )
        }

        let mut pkg = Package::new();
        if !(check_smt(&self.options, CLIOptions::NoInteractive)) {
            let x = Select::new("Choose one type of package: ", TYPES.to_vec())
                .prompt()
                .unwrap();

            pkg.name = Text::new("Name: ")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. my_example")
                .prompt()
                .unwrap();

            pkg.version = Version::parse(
                Text::new("Version: ")
                    .with_validator(required!("This field is required"))
                    .with_validator(&|obj: &str| {
                        return match Version::parse(&obj) {
                            Ok(_) => Ok(Validation::Valid),
                            Err(_) => Ok(Validation::Invalid(
                                "A correct version must be types (check semVer)".into(),
                            )),
                        };
                    })
                    .with_help_message("e.g. 1.0.0 (SemVer)")
                    .with_default("1.0.0")
                    .prompt()
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            pkg.entrypoint = Text::new("Entrypoint: ")
                .with_validator(required!("This field is required"))
                .with_help_message("e.g. ./main.typ")
                .with_default("./main.typ")
                .prompt()
                .unwrap();

            if x == TYPES[1] || x == TYPES[2] {
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
                        .with_default("Unlicense")
                        .with_validator(&|obj: &str| match spdx::Expression::parse(obj) {
                            Ok(val) => {
                                for x in val.requirements() {
                                    let id = x.req.license.id().unwrap();
                                    if !id.is_osi_approved() {
                                        return Ok(Validation::Invalid(
                                            "It must be an OSI approved!".into(),
                                        ));
                                    }
                                }
                                Ok(Validation::Valid)
                            }
                            Err(_) => Ok(Validation::Invalid("Can't parse your expression".into())),
                        })
                        .prompt()
                        .unwrap(),
                );

                pkg.description = Some(
                    Text::new("description: ")
                        .with_help_message("e.g. A package")
                        .prompt()
                        .unwrap(),
                );

                if x == TYPES[2] {
                    pkg.repository = Some(
                        Text::new("URL of the repository: ")
                            .with_help_message("e.g. https://github.com/ThumusLive/utpm")
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
        } else {
            let _ = &self.options.pop_front();
            if force {
                let _ = &self.options.pop_front();
            }
            if !check_smt(&self.options, CLIOptions::Name) {
                return Err(ErrorState::UnknowError("Need the param --name".to_string()));
            }
            while self.options.len() > 0 {
                let x = self.options.pop_front().unwrap();
                let y = match self.options.pop_front().unwrap() {
                    CLIOptions::Token(content) => content,
                    _ => return Err(ErrorState::UnknowError("Unknown Token".to_string())),
                };
                match x {
                    CLIOptions::Name => pkg.name = y,
                    CLIOptions::SelectVersion => pkg.version = Version::parse(y.as_str()).unwrap(),
                    CLIOptions::Entrypoint => pkg.entrypoint = y,
                    CLIOptions::Author => {
                        if let Some(ref mut vector) = pkg.authors {
                            vector.push(y);
                        } else {
                            pkg.authors = Some(vec![y]);
                        }
                    }
                    CLIOptions::License => match spdx::Expression::parse(&y) {
                        Ok(val) => {
                            for x in val.requirements() {
                                let id = x.req.license.id().unwrap();
                                if !id.is_osi_approved() {
                                    return Err(ErrorState::UnknowError(String::from(
                                        "It must be an OSI approved!",
                                    )));
                                }
                            }
                            pkg.license = Some(y)
                        }
                        Err(_) => {
                            return Err(ErrorState::UnknowError(
                                "Can't parse your expression".to_string(),
                            ))
                        }
                    },
                    CLIOptions::Description => pkg.description = Some(y),
                    CLIOptions::Repository => pkg.repository = Some(y),
                    CLIOptions::Homepage => pkg.homepage = Some(y),

                    CLIOptions::Keyword => {
                        if let Some(ref mut vector) = pkg.keywords {
                            vector.push(y);
                        } else {
                            pkg.authors = Some(vec![y]);
                        }
                    }
                    CLIOptions::Compiler => pkg.compiler = Some(y),

                    CLIOptions::Exclude => {
                        if let Some(ref mut vector) = pkg.exclude {
                            vector.push(y);
                        } else {
                            pkg.authors = Some(vec![y]);
                        }
                    }
                    _ => return Err(ErrorState::UnknowError("Bad options passed".to_string())),
                }
            }
        }
        TypstConfig::new(pkg).write(&typ);
        Ok(GoodState::Good("File created!".bold().to_string()))
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
        println!("  --force, -f                             Force the creation of the file");
    }
}
