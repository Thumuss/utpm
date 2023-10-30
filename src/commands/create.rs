use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use colored::Colorize;

use inquire::{list_option::ListOption, required, validator::Validation, MultiSelect, Text};
use semver::Version;
use serde_json::json;

use crate::utils::{
    paths::{check_path_file, get_current_dir},
    state::{Responses, Result},
    Extra, Package, TypstConfig,
};

static TYPES: [&str; 4] = ["Local", "Public", "More options", "Namespace"];

pub fn run(
    force: &bool,
    cli: &bool,
    mut pkg: Package,
    mut extra: Extra,
    populate: &bool,
    mut res: Responses
) -> Result<Responses> {
    let curr = get_current_dir()?;
    let typ = curr.clone() + "/typst.toml";
    if check_path_file(&typ) && !force {
        res.push(json!({
            "message": "Nothing to do"
        }));
        return Ok(res);
    }

    if *force {
        res.push(json!({
            "message": format!(
                "{} {}",
                "WARNING:".bold().yellow(),
                "--force is a dangerous flag, use it cautiously".bold()
            )
        }));
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
        .prompt()?;

        let public = choose_options.contains(&TYPES[1]);
        let more = choose_options.contains(&TYPES[2]);
        let extra_opts = choose_options.contains(&TYPES[3]);

        pkg.name = Text::new("Name: ")
            .with_validator(required!("This field is required"))
            .with_help_message("e.g. my_example")
            .prompt()?;

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
                .prompt()?
                .as_str(),
        )?;

        pkg.entrypoint = Text::new("Entrypoint: ")
            .with_validator(required!("This field is required"))
            .with_help_message("e.g. main.typ")
            .with_default("main.typ")
            .prompt()?;

        if public {
            pkg.authors = Some(
                Text::new("Authors: ")
                    .with_help_message("e.g. Thumus,Somebody,Somebody Else")
                    .prompt()?
                    .split(",")
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>(),
            );

            pkg.license = Some(
                Text::new("License: ")
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
                    .prompt()?,
            );

            pkg.description = Some(
                Text::new("description: ")
                    .with_help_message("e.g. A package")
                    .prompt()?,
            );
        }
        if more {
            pkg.repository = Some(
                Text::new("URL of the repository: ")
                    .with_help_message("e.g. https://github.com/ThumusLive/utpm")
                    .prompt()?,
            );
            pkg.homepage = Some(
                Text::new("Homepage: ")
                    .with_help_message("e.g. anything")
                    .prompt()?,
            );
            pkg.keywords = Some(
                Text::new("Keywords: ")
                    .with_help_message("e.g. Typst,keyword")
                    .prompt()?
                    .split(",")
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>(),
            );
            pkg.compiler = Some(Version::parse(
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
                    .prompt()?
                    .as_str(),
            )?);
            pkg.exclude = Some(
                Text::new("Exclude: ")
                    .with_help_message("e.g. backup/mypassword.txt,.env")
                    .prompt()?
                    .split(",")
                    .filter(|f| f.len() > 0)
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>(),
            );
        }

        if extra_opts {
            extra.namespace = Some(
                Text::new("Namespace: ")
                    .with_help_message("e.g. backup/mypassword.txt,.env")
                    .with_default("local")
                    .prompt()?
                    .to_string(),
            )
        }
    }

    if *populate {
        let mut file = File::create(curr.clone() + "/README.md")?; // README.md
        file.write_all(("# ".to_string() + pkg.name.clone().as_str()).as_bytes())?;
        if let Some(license) = &pkg.license {
            if let Some(exp) = spdx::license_id(license.as_str()) {
                file = File::create(curr.clone() + "/LICENSE")?; // LICENSE
                file.write_all(exp.text().as_bytes())?;
            }
        }
        create_dir_all(curr.clone() + "/examples")?; // examples
        let examples = curr.clone() + "/examples";
        file = File::create(examples + "/tests.typ")?; // examples/texts.typ
        let fm = format!(
            "#import \"@{}/{}:{}\": *\nDo...",
            extra.namespace.clone().unwrap_or("preview".to_string()),
            pkg.name.clone(),
            pkg.version.clone().to_string()
        );
        file.write_all(fm.as_bytes())?;
        file = File::create(pkg.entrypoint.clone())?; // main.typ
        file.write_all(b"// This file is generated by UTPM (https://github.com/ThumusLive/utpm)")?;
    }

    TypstConfig::new(pkg, extra).write(&typ); // typst.toml
    println!("{}", "File created to {typ}".bold().to_string());
    Ok(res)
}
