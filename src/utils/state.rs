use colored::Colorize;
use std::fmt;

pub enum GoodState {
    Good(String),
    None,
}

pub enum ErrorState {
    UnknowError(String),

    CurrentDirectoryError(String),
    CreationDirectoryError(String),

    UnexpectedTokenError(String),
    NoneTokenError(String),
}

pub type Result<T> = std::result::Result<T, ErrorState>;
pub type GoodResult = std::result::Result<GoodState, ErrorState>;

impl fmt::Display for ErrorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorState::UnknowError(string) => write!(f, "{}: {string}", "Error".red().bold()),
            ErrorState::CurrentDirectoryError(string) => {
                write!(f, "{}: {string}", "Current Directory Error".red().bold(),)
            }

            ErrorState::CreationDirectoryError(string) => {
                write!(f, "{}: {string}", "Creation Directory Error".red().bold(),)
            }

            ErrorState::UnexpectedTokenError(string) => {
                write!(f, "{}: {string}", "Unexpected Token Error".red().bold(),)
            }

            ErrorState::NoneTokenError(string) => {
                write!(f, "{}: {string}", "None Token Error".red().bold(),)
            }
        }
    }
}

impl From<std::io::Error> for ErrorState {
    fn from(err: std::io::Error) -> ErrorState {
        ErrorState::UnknowError(err.to_string())
    }
}

impl From<inquire::InquireError> for ErrorState {
    fn from(err:  inquire::InquireError) -> ErrorState {
        ErrorState::UnknowError(err.to_string())
    }   
}