extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

mod common;
use common::input_text;

#[test]
fn when_buffer_is_empty_i_enters_insert_mode() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char('i')).unwrap();

    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn when_there_is_content_i_does_not_move_the_cursor() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char('i')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(Key::Esc).unwrap();
    ed.handle_key(Key::Char('i')).unwrap();
    input_text(&mut ed, "x");

    assert_eq!(ed.window().content_view().line(0), "123x4");
}

#[test]
fn when_buffer_is_empty_a_enters_insert_mode() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char('a')).unwrap();

    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn when_there_is_content_a_moves_the_cursor_forward() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char('a')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(Key::Esc).unwrap();
    ed.handle_key(Key::Char('a')).unwrap();
    input_text(&mut ed, "x");

    assert_eq!(ed.window().content_view().line(0), "1234x");
}

#[test]
fn enter_to_prompt_with_colon() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char(':')).unwrap();

    assert_eq!(ed.mode(), Mode::Prompt);
}
