use colored::Colorize;

use inquire::{list_option::ListOption, required, validator::Validation, MultiSelect, Text};
use semver::Version;

use crate::utils::{
    paths::{check_path_file, get_current_dir},
    state::{GoodResult, GoodState},
    Extra, Package, TypstConfig,
};

static TYPES: [&str; 4] = ["Local", "Public", "More options", "Namespace"];

pub fn run(force: &bool, cli: &bool, mut pkg: Package, mut extra: Extra) -> GoodResult {
    let typ = get_current_dir()? + "/typst.toml";
    if check_path_file(&typ) && !force {
        return Ok(GoodState::None);
    }

    if *force {
        println!(
            "{} {}",
            "WARNING:".bold().yellow(),
            "--force is a dangerous flag, use it cautiously".bold()
        )
    }

    if !cli {
        let choose_options = MultiSelect::new(
            "Choose between local and public package and options: ",
            TYPES.to_vec(),
        )
        .with_validator(|a: &[ListOption<&&str>]| {
            let x = a
                .iter()
                .any(|o| *o.value == "Local" || *o.value == "Public");
            match x {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid(
                    "Remember to chose between public and local".into(),
                )),
            }
        })
        .prompt()
        .unwrap();

        let public = choose_options.contains(&TYPES[1]);
        let more = choose_options.contains(&TYPES[2]);
        let extra_opts = choose_options.contains(&TYPES[3]);

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

        if public {
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
        }
        if more {
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
                Version::parse(
                    Text::new("Compiler version required: ")
                        .with_help_message("e.g. 0.7.0")
                        .with_validator(&|obj: &str| {
                            return match Version::parse(&obj) {
                                Ok(_) => Ok(Validation::Valid),
                                Err(_) => Ok(Validation::Invalid(
                                    "A correct version must be types (check semVer)".into(),
                                )),
                            };
                        })
                        .prompt()
                        .unwrap()
                        .as_str(),
                )
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

        if extra_opts {
            extra.namespace = Some(Text::new("Namespace: ")
                .with_help_message("e.g. backup/mypassword.txt,.env")
                .with_default("local")
                .prompt()
                .unwrap()
                .to_string())
        }
    }
    TypstConfig::new(pkg, extra).write(&typ);
    Ok(GoodState::Message(format!("File created to {typ}").bold().to_string()))
}
