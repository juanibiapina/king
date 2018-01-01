extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

mod common;
use common::input_text;

#[test]
fn typing_text() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":abc");

    assert_eq!(ed.prompt().command_text(), ":abc");
}

#[test]
fn deleting_text() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":abc");
    ed.handle_key(&Key::Backspace).unwrap();

    assert_eq!(ed.prompt().command_text(), ":ab");
}

#[test]
fn deleting_last_character() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":a");
    ed.handle_key(&Key::Backspace).unwrap();
    ed.handle_key(&Key::Backspace).unwrap();

    assert_eq!(ed.mode(), Mode::Normal);
    assert_eq!(ed.prompt().command_text(), "");
    assert_eq!(ed.prompt().cur_x, 0);
}

#[test]
fn leave_prompt_with_escape() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char(':')).unwrap();
    ed.handle_key(&Key::Esc).unwrap();

    assert_eq!(ed.mode(), Mode::Normal);
    assert_eq!(ed.prompt().command_text(), "");
    assert_eq!(ed.prompt().cur_x, 0);
}

#[test]
fn running_command_to_quit() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":quit");
    ed.handle_key(&Key::Enter).unwrap();

    assert_eq!(ed.running(), false);
}
