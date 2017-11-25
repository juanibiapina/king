extern crate king;

use king::editor::Editor;
use king::input::Key;

mod common;
use common::input_text;

#[test]
fn simple_navigation_with_hjkl() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(Key::Enter).unwrap();
    assert_eq!(ed.cursor(), (0, 0));

    ed.handle_key(Key::Char('j')).unwrap();
    assert_eq!(ed.cursor(), (1, 0));

    ed.handle_key(Key::Char('l')).unwrap();
    assert_eq!(ed.cursor(), (1, 1));

    ed.handle_key(Key::Char('h')).unwrap();
    assert_eq!(ed.cursor(), (1, 0));

    ed.handle_key(Key::Char('k')).unwrap();
    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn content_boundary_checks() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(Key::Enter).unwrap();
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
