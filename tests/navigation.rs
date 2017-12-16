extern crate king;

use king::editor::Editor;
use king::input::Key;

mod common;
use common::input_text;

#[test]
fn simple_navigation_with_hjkl() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();
    assert_eq!(ed.cursor(), (0, 0));

    ed.handle_key(&Key::Char('j')).unwrap();
    assert_eq!(ed.cursor(), (1, 0));

    ed.handle_key(&Key::Char('l')).unwrap();
    assert_eq!(ed.cursor(), (1, 1));

    ed.handle_key(&Key::Char('h')).unwrap();
    assert_eq!(ed.cursor(), (1, 0));

    ed.handle_key(&Key::Char('k')).unwrap();
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn navigating_over_emoticons() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, "i");
    input_text(&mut ed, "a😀😀a");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.cursor(), (0, 5));

    input_text(&mut ed, "h");
    assert_eq!(ed.cursor(), (0, 3));

    input_text(&mut ed, "h");
    assert_eq!(ed.cursor(), (0, 1));

    input_text(&mut ed, "l");
    assert_eq!(ed.cursor(), (0, 3));

    input_text(&mut ed, "l");
    assert_eq!(ed.cursor(), (0, 5));
}

#[test]
fn boundary_when_line_ends_with_emoticon() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, "i");
    input_text(&mut ed, "a😀😀");
    ed.handle_key(&Key::Esc).unwrap();
    assert_eq!(ed.cursor(), (0, 3));

    input_text(&mut ed, "l");
    assert_eq!(ed.cursor(), (0, 3));
}

#[test]
fn content_boundary_checks() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();
    assert_eq!(ed.cursor(), (0, 0));

    input_text(&mut ed, "jjjj");
    assert_eq!(ed.cursor(), (2, 0));

    input_text(&mut ed, "llllll");
    assert_eq!(ed.cursor(), (2, 5));

    input_text(&mut ed, "hhhhhhhh");
    assert_eq!(ed.cursor(), (2, 0));

    input_text(&mut ed, "kkkkk");
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn moving_up_to_shorter_line() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "12");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('o')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();

    input_text(&mut ed, "k");
    assert_eq!(ed.cursor(), (0, 1));
}

#[test]
fn moving_down_to_shorter_line() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('o')).unwrap();
    input_text(&mut ed, "12");
    ed.handle_key(&Key::Esc).unwrap();

    input_text(&mut ed, "kllj");
    assert_eq!(ed.cursor(), (1, 1));
}

#[test]
fn moving_up_to_middle_of_emoticon() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "12😀");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('o')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();

    input_text(&mut ed, "k");
    assert_eq!(ed.cursor(), (0, 2));
}

#[test]
fn moving_down_to_middle_of_emoticon() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(&Key::Char('i')).unwrap();
    input_text(&mut ed, "1234");
    ed.handle_key(&Key::Esc).unwrap();
    ed.handle_key(&Key::Char('o')).unwrap();
    input_text(&mut ed, "12😀");
    ed.handle_key(&Key::Esc).unwrap();

    input_text(&mut ed, "klj");
    assert_eq!(ed.cursor(), (1, 2));
}
