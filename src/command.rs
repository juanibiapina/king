use error::{Error, Result};

#[derive(Clone)]
pub enum Command {
    Quit,
    Edit(String),
    Write,
    EnterPrompt(char),
    CancelPrompt,
    RunPrompt,
    EnterInsert,
    EnterInsertAfterCursor,
    OpenLineAfter,
    OpenLineBefore,
    LeaveInsert,
    DeleteCharBeforeCursor,
    DeleteCharBeforeCursorInPrompt,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
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
