pub mod utils;
pub mod commands;

use clap::{Parser, Subcommand};
use commands::{create, link, list};
use utils::Package;
use utils::state::GoodState;

#[derive(Parser)]
#[command(author = "Thumus", version = "0.1.0")]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a file for your project (typst.toml)
    Create {
        /// Desactivate interactive session
        #[arg(short='m', long, requires = "ni")]
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
        #[arg(short, long, default_value_t=String::from("./main.typ"))]
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
        #[arg(short='x',long)]
        exclude: Option<Vec<String>>,
    },
    /// Link your project to your dirs
    Link {
        /// Force the copy of the dir / creation of the symlink
        #[arg(short, long)]
        force: bool,

        /// Will create a symlink instead of copying
        #[arg(short, long)]
        no_copy: bool
    },
    /// List all of the package in the local folder
    List {},
}
fn main() {
    let x = Cli::parse();
    let res = match &x.command {
        Commands::Create { cli, force, name, version, entrypoint, authors, license, description, repository, homepage, keywords, compiler, exclude } => {
            let pkg: Package = Package { name: name.clone().unwrap_or("".to_string()), version: version.clone(), entrypoint: entrypoint.clone(), authors: authors.clone(), license: license.clone(), description: description.clone(), repository: repository.clone(), homepage: homepage.clone(), keywords: keywords.clone(), compiler: compiler.clone(), exclude: exclude.clone() };
            create::create(*force, *cli, pkg)
        },
        Commands::Link { force, no_copy } => {
            link::link(*force, *no_copy)
        }
        Commands::List {  } => {
            list::list()
        }
    };

    match res {
        Ok(val) => match val {
            GoodState::None => (),
            GoodState::Good(string) => println!("{}", string) 
        },

        Err(val) => println!("{}", val.to_string())
    }
}
