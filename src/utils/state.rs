use colored::Colorize;

pub enum GoodState {
    Good(String),
    Help,
    NothingToDo(String)
}

pub enum ErrorState {
    UnknowError(String),

    CurrentDirectoryError(String),
    CreationDirectoryError(String),

    UnexpectedTokenError(String),
    NoneTokenError(String),

    TypstCompileError(String),
    GitCloneError(String),

    CreatingCommandError(String),

    SymlinkUnixError(String),
}

impl ErrorState {
    pub fn display(&self) {
        match self {
            ErrorState::UnknowError(string) => eprintln!("{}: {string}", "Error".red().bold()),
            ErrorState::CurrentDirectoryError(string) => {
                eprintln!("{}: {string}", "Current Directory Error".red().bold(),)
            }
            ErrorState::CreationDirectoryError(string) => {
                eprintln!("{}: {string}", "Creation Directory Error".red().bold(),)
            }
            ErrorState::UnexpectedTokenError(string) => {
                eprintln!("{}: {string}", "Unexpected Token Error".red().bold(),)
            }
            ErrorState::NoneTokenError(string) => {
                eprintln!("{}: {string}", "None Token Error".red().bold(),)
            }
            ErrorState::TypstCompileError(string) => {
                eprintln!("{}: {string}", "Compile Error".red().bold(),)
            }
            ErrorState::GitCloneError(string) => {
                eprintln!("{}: {string}", "Git Clone Error".red().bold(),)
            }
            ErrorState::CreatingCommandError(string) => {
                eprintln!("{}: {string}", "Creation Command Error".red().bold(),)
            }
            ErrorState::SymlinkUnixError(string) => {
                eprintln!("{}: {string}", "Symlink Unix Error".red().bold(),)
            }
        }
    }
}
