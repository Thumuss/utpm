pub mod commands;
pub mod utils;

use clap::{Parser, Subcommand};
use commands::{create, install, link, list, unlink};
use utils::state::GoodState;
use utils::{paths::d_packages, Extra, Package};

#[derive(Parser)]
#[command(author = "Thumus", version = "1.1.0")]

/// An unofficial typst package manager for your projects.
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a file for your project (typst.toml)
    Create {
        /// Desactivate interactive session
        #[arg(short = 'm', long, requires = "ni")]
        cli: bool,

        /// Force the creation of a file
        #[arg(short, long)]
        force: bool,

        /// Name of the project
        #[arg(short, long, group = "ni")]
        name: Option<String>,

        /// Version of the project
        #[arg(short, long, default_value_t=semver::Version::parse("1.0.0").unwrap())]
        version: semver::Version,

        /// Path to the main file of the project
        #[arg(short, long, default_value_t=String::from("main.typ"))]
        entrypoint: String,

        /// Authors of the project
        #[arg(short, long)]
        authors: Option<Vec<String>>,

        /// License
        #[arg(short, long)]
        license: Option<String>,

        /// A little description
        #[arg(short, long)]
        description: Option<String>,

        /// The link to your repository
        #[arg(short, long)]
        repository: Option<String>,

        /// Link to your homepage
        #[arg(short, long)]
        homepage: Option<String>,

        /// Keywords to find your project
        #[arg(short, long)]
        keywords: Option<Vec<String>>,

        /// CMinimum compiler version
        #[arg(short, long)]
        compiler: Option<semver::Version>,

        /// Excludes files
        #[arg(short = 'x', long)]
        exclude: Option<Vec<String>>,

        /// Excludes files
        #[arg(short = 'N', long)]
        namespace: Option<String>,
    },
    /// Link your project to your dirs
    Link {
        /// Force the copy of the dir / creation of the symlink
        #[arg(short, long)]
        force: bool,

        /// Will create a symlink instead of copying
        #[arg(short, long)]
        no_copy: bool,
    },
    /// List all of the package in the local folder
    List {},
    /// Display path to typst packages folder
    PackagesPath,

    /// Delete package previously install with utpm
    Unlink {
        /// The name of the package
        name: Option<String>,

        /// Namespace, where your packages are install (default local)
        #[arg(short, long)]
        namespace: Option<String>,

        /// Do you want to delete the namespace or not
        #[arg(short, long)]
        delete_namespace: bool,

        /// The version you want to delete, if nothing has been provided it will be the whole package
        #[arg(short, long)]
        version: Option<semver::Version>,

        /// Confirm the deletion of a dir
        #[arg(short, long)]
        yes: bool,
    },

    /// Delete a bunch of packages
    BulkDelete {
        /// Names of your packages, use version with this syntax: mypackage:1.0.0
        names: Vec<String>,

        /// The namespace you want to bulk-delete
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Install all dependencies from the `typst.toml`
    Install {
        /// If you want to install a specific package
        url: Option<String>,

        /// Passed force to all link commands
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}
fn main() {
    let x = Cli::parse();
    let res = match &x.command {
        Commands::Create {
            cli,
            force,
            name,
            version,
            entrypoint,
            authors,
            license,
            description,
            repository,
            homepage,
            keywords,
            compiler,
            exclude,
            namespace,
        } => {
            let pkg: Package = Package {
                name: name.clone().unwrap_or("".to_string()),
                version: version.clone(),
                entrypoint: entrypoint.clone(),
                authors: authors.clone(),
                license: license.clone(),
                description: description.clone(),
                repository: repository.clone(),
                homepage: homepage.clone(),
                keywords: keywords.clone(),
                compiler: compiler.clone(),
                exclude: exclude.clone(),
            };
            let mut extra: Extra = Extra::new();
            extra.namespace = namespace.clone();
            create::run(force, cli, pkg, extra)
        }
        Commands::Link { force, no_copy } => link::run(*force, *no_copy, None),
        Commands::List {} => list::run(),
        Commands::PackagesPath => {
            println!("Packages are located at: '{}'", d_packages());
            Ok(utils::state::GoodState::None)
        }
        Commands::Unlink {
            name,
            version,
            namespace,
            yes,
            delete_namespace,
        } => unlink::run(
            name,
            version.clone(),
            namespace.clone(),
            yes,
            delete_namespace,
        ),
        Commands::BulkDelete { names, namespace } => {
            for name in names {
                let name_and_version = name.split(":").collect::<Vec<&str>>();
                match unlink::run(
                    &Some(name_and_version[0].to_string()),
                    if name_and_version.len() > 0 {
                        Some(semver::Version::parse(name_and_version[1]).unwrap())
                    } else {
                        None
                    },
                    namespace.clone(),
                    &true,
                    &false,
                ) {
                    Ok(_) => None,
                    Err(err) => {
                        println!("{}", err.to_string());
                        Some(err)
                    }
                };
            }
            Ok(GoodState::None)
        }
        Commands::Install { url, force } => install::run(force.clone(), url.as_ref()),
    };

    match res {
        Ok(val) => match val {
            GoodState::None => (),
            GoodState::Message(string) => println!("{}", string),
        },

        Err(val) => println!("{}", val.to_string()),
    }
}
