use error::{Error, Result};

pub enum Command {
    Quit,
}

impl Command {
    pub fn parse(text: &str) -> Result<Command> {
        match text {
            "quit" => Ok(Command::Quit),
            _ => Err(Error::CommandNotFound(text.to_owned())),
        }
    }
}
