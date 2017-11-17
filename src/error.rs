use std::result;

#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
}

pub type Result<T> = result::Result<T, Error>;

pub fn error_message(err: Error) -> String {
    match err {
        Error::CommandNotFound(name) => format!("Command not found: {}", name),
    }
}
