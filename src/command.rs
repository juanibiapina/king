use error::{Error, Result};

pub enum Command {
    Quit,
    Edit(String),
    Write,
}

impl Command {
    pub fn parse(text: &str) -> Result<Command> {
        let words = text.split(" ").collect::<Vec<_>>();

        match words[0] {
            ":quit" => Ok(Command::Quit),
            ":write" => Ok(Command::Write),
            ":edit" => Ok(Command::Edit(words[1].to_owned())),
            _ => Err(Error::CommandNotFound(text.to_owned())),
        }
    }
}
