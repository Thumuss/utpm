use colored::Colorize;
use std::{fmt, io::Error as IError};
pub enum GoodState {
    Message(String),
    None,
}

pub enum ErrorState {
    UnknowError(String),

    CurrentDirectoryError(String),
    CreationDirectoryError(String),
    HomeDirectoryError(String),

    UnexpectedIOError(String),
    UnexpectedQuestionsError(String),
    UnexpectedGitError(String),
    UnexpectedSemVerError(String),

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

            ErrorState::HomeDirectoryError(string) => {
                write!(f, "{}: {string}", "Home Directory Error".red().bold(),)
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
            ErrorState::UnexpectedIOError(string) => {
                write!(f, "{}: {string}", "Unexpected IO Error".red().bold(),)
            }
            ErrorState::UnexpectedQuestionsError(string) => {
                write!(f, "{}: {string}", "Unexpected Questions Error".red().bold(),)
            }
            ErrorState::UnexpectedGitError(string) => {
                write!(f, "{}: {string}", "Unexpected Git Error".red().bold(),)
            }
            ErrorState::UnexpectedSemVerError(string) => {
                write!(f, "{}: {string}", "Unexpected SemVer Error".red().bold(),)
            }
        }
    }
}

impl From<IError> for ErrorState {
    fn from(err: IError) -> ErrorState {
        ErrorState::UnexpectedIOError(err.to_string())
    }
}

impl From<inquire::InquireError> for ErrorState {
    fn from(err: inquire::InquireError) -> ErrorState {
        ErrorState::UnexpectedQuestionsError(err.to_string())
    }
}

impl From<git2::Error> for ErrorState {
    fn from(err: git2::Error) -> ErrorState {
        ErrorState::UnexpectedGitError(err.to_string())
    }
}

impl From<semver::Error> for ErrorState {
    fn from(err: semver::Error) -> ErrorState {
        ErrorState::UnexpectedSemVerError(err.to_string())
    }
}
