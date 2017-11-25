extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

mod common;
use common::input_text;

fn editor() -> Editor {
    let mut ed = Editor::new(10, 10);
    ed.handle_key(Key::Char('i')).unwrap();
    return ed;
}

#[test]
fn escape_leaves_insert_mode() {
    let mut ed = editor();

    ed.handle_key(Key::Esc).unwrap();

    assert_eq!(ed.mode, Mode::Normal);
}

#[test]
fn entering_text_moves_the_cursor() {
    let mut ed = editor();

    input_text(&mut ed, "some text");

    assert_eq!(ed.get_cursor(), (0, 9));
}

#[test]
fn entering_text_adds_the_text_to_the_buffer() {
    let mut ed = editor();

    input_text(&mut ed, "some text");

    assert_eq!(ed.window.content_view().line(0), "some text");
}

#[test]
fn entering_wide_chars_moves_the_cursor() {
    let mut ed = editor();

    input_text(&mut ed, "😀😀");

    assert_eq!(ed.get_cursor(), (0, 4));
}

#[test]
fn deleting_text_with_backspace() {
    let mut ed = editor();

    input_text(&mut ed, "some text");
    ed.handle_key(Key::Backspace).unwrap();
    ed.handle_key(Key::Backspace).unwrap();

    assert_eq!(ed.get_cursor(), (0, 7));
    assert_eq!(ed.window.content_view().line(0), "some te");
}
