use colored::Colorize;

use inquire::{required, validator::Validation, Select, Text};
use semver::Version;

use crate::utils::{
    paths::{check_path_file, get_current_dir},
    state::{GoodResult, GoodState},
    Package, TypstConfig,
};

static TYPES: [&str; 3] = ["Local", "Public", "Public with more options"];

pub fn create(force: bool, cli: bool, mut pkg: Package) -> GoodResult {
    let typ = get_current_dir()? + "/typst.toml";
    if check_path_file(&typ) && !force {
        return Ok(GoodState::None);
    }

    if force {
        println!(
            "{} {}",
            "WARNING:".bold().yellow(),
            "--force is a dangerous flag, use it cautiously".bold()
        )
    }

    if !cli {
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
        }
    }
    TypstConfig::new(pkg).write(&typ);
    Ok(GoodState::Good("File created!".bold().to_string()))
}
