use std::result;
use std::io;

#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
    NoFileName,
    IoError(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub fn error_message(err: Error) -> String {
    match err {
        Error::CommandNotFound(name) => format!("Command not found: {}", name),
        Error::NoFileName => format!("No file name given"),
        Error::IoError(io_error) => format!("IO error: {}", io_error),
    }
}
