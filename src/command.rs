use error::{Error, Result};

pub enum Command {
    Quit,
    Edit(String),
}

impl Command {
    pub fn parse(text: &str) -> Result<Command> {
        let words = text.split(" ").collect::<Vec<_>>();

        match words[0] {
            "quit" => Ok(Command::Quit),
            "edit" => Ok(Command::Edit(words[1].to_owned())),
            _ => Err(Error::CommandNotFound(text.to_owned())),
        }
    }
}
