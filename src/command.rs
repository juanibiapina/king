use error::{Error, Result};
use editor::Editor;
use prompt;

#[derive(Clone)]
pub enum Command {
    Quit,
    Edit(String),
    Write,
    EnterPrompt(char),
    CancelPrompt,
    RunPrompt,
    EnterInsert,
    LeaveInsert,
    DeleteCharBeforeCursor,
    DeleteCharBeforeCursorInPrompt,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
}

pub fn run(cmd: &Command, ed: &mut Editor) -> Result<()> {
  match *cmd {
    Command::Quit => ed.exit(),
    Command::Write => ed.write(),
    Command::Edit(ref filename) => ed.edit(filename),
    Command::EnterPrompt(c) => ed.enter_prompt(c),
    Command::CancelPrompt => ed.cancel_prompt(),
    Command::RunPrompt => ed.run_prompt(),
    Command::EnterInsert => ed.enter_insert(),
    Command::LeaveInsert => ed.leave_insert(),
    Command::DeleteCharBeforeCursor => ed.window.delete_char(),
    Command::DeleteCharBeforeCursorInPrompt => prompt::delete_char(ed),
    Command::MoveCursorLeft => ed.window.move_cursor(0, -1),
    Command::MoveCursorRight => ed.window.move_cursor(0, 1),
    Command::MoveCursorUp => ed.window.move_cursor(-1, 0),
    Command::MoveCursorDown => ed.window.move_cursor(1, 0),
  }
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
