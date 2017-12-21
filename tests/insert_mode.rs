extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

mod common;
use common::input_text;

fn editor() -> Editor {
    let mut ed = Editor::new(10, 10);
    ed.handle_key(&Key::Char('i')).unwrap();
    ed
}

#[test]
fn enter_and_exit() {
    let mut ed = editor();

    ed.handle_key(&Key::Esc).unwrap();

    assert_eq!(ed.mode(), Mode::Normal);
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn entering_text_moves_the_cursor() {
    let mut ed = editor();

    input_text(&mut ed, "some text");

    assert_eq!(ed.cursor(), (0, 9));
}

#[test]
fn entering_text_adds_the_text_to_the_buffer() {
    let mut ed = editor();

    input_text(&mut ed, "some text");

    assert_eq!(ed.window().content_view().line(0), "some text");
}

#[test]
fn entering_wide_chars_moves_the_cursor() {
    let mut ed = editor();

    input_text(&mut ed, "😀😀");

    assert_eq!(ed.cursor(), (0, 4));
}

#[test]
fn deleting_text_with_backspace() {
    let mut ed = editor();

    input_text(&mut ed, "some 😀a😀");
    assert_eq!(ed.cursor(), (0, 10));

    ed.handle_key(&Key::Backspace).unwrap();
    assert_eq!(ed.cursor(), (0, 8));
    assert_eq!(ed.window().content_view().line(0), "some 😀a");

    ed.handle_key(&Key::Backspace).unwrap();
    assert_eq!(ed.cursor(), (0, 7));
    assert_eq!(ed.window().content_view().line(0), "some 😀");

    ed.handle_key(&Key::Backspace).unwrap();
    assert_eq!(ed.cursor(), (0, 5));
    assert_eq!(ed.window().content_view().line(0), "some ");
}

#[test]
fn deleting_lines_with_backspace() {
    let mut ed = editor();

    input_text(&mut ed, "12");
    ed.handle_key(&Key::Esc).unwrap();
    input_text(&mut ed, "o");
    input_text(&mut ed, "34");
    ed.handle_key(&Key::Esc).unwrap();
    input_text(&mut ed, "h");
    input_text(&mut ed, "i");
    ed.handle_key(&Key::Backspace).unwrap();

    assert_eq!(ed.cursor(), (0, 2));
    assert_eq!(ed.window().content_view().line(0), "1234");
    assert_eq!(ed.window().content_view().height(), 1);
}

#[test]
fn when_buffer_is_empty_i_enters_insert_mode() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();

    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn when_there_is_content_i_does_not_move_the_cursor() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "x");

    assert_eq!(ed.window().content_view().line(0), "123x4");
}

#[test]
fn inserting_text_when_there_are_widechars() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "1😀ab");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "x");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.window().content_view().line(0), "1😀axb");

    ed.handle_key(&Key::Char('h')).unwrap();
    ed.handle_key(&Key::Char('h')).unwrap();
    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "x");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.window().content_view().line(0), "1😀xaxb");

    ed.handle_key(&Key::Char('h')).unwrap();
    ed.handle_key(&Key::Char('h')).unwrap();
    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "x");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.window().content_view().line(0), "1x😀xaxb");

    ed.handle_key(&Key::Char('h')).unwrap();
    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "y");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.window().content_view().line(0), "1yx😀xaxb");
}

#[test]
fn when_buffer_is_empty_a_enters_insert_mode() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('a')).unwrap();

    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn when_there_is_content_a_moves_the_cursor_forward() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('a')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('a')).unwrap();
    input_text(&mut ed, "x");

    assert_eq!(ed.window().content_view().line(0), "1234x");
}

#[test]
fn o_opens_insert_mode_on_the_next_line() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "asdf");
    ed.handle_key(&Key::Esc).unwrap();

    ed.handle_key(&Key::Char('o')).unwrap();
    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (1, 0));
}

#[test]
fn shift_o_opens_insert_mode_on_the_previous_line() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('O')).unwrap();
    assert_eq!(ed.mode(), Mode::Insert);
    assert_eq!(ed.cursor(), (0, 0));
    assert_eq!(ed.window().content_view().line(0), "");
    assert_eq!(ed.window().content_view().line(1), "1234");
}
