extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

mod common;
use common::input_text;

#[test]
fn editing_a_file_that_exists() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();

    assert_eq!(ed.mode(), Mode::Normal);
    assert_eq!(ed.prompt.message, Some("\"tests/fixtures/file_with_contents\"".to_owned()));
}

#[test]
fn cursor_position_reset_when_editing_a_new_file() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();
    ed.handle_key(&Key::Char('j')).unwrap();
    input_text(&mut ed, ":edit tests/fixtures/file2");
    ed.handle_key(&Key::Enter).unwrap();

    assert_eq!(ed.cursor(), (0, 0));
}

#[test]
fn when_content_fits_the_window() {
    let mut ed = Editor::new(10, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();

    assert_eq!(ed.window().content_view().height(), 3);
    assert_eq!(ed.window().content_view().line(0), "line 1");
    assert_eq!(ed.window().content_view().line(1), "line 2");
    assert_eq!(ed.window().content_view().line(2), "line 3");
}

#[test]
fn when_content_is_larger_than_the_window_height() {
    let mut ed = Editor::new(3, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();

    assert_eq!(ed.window().content_view().height(), 2);
    assert_eq!(ed.window().content_view().line(0), "line 1");
    assert_eq!(ed.window().content_view().line(1), "line 2");
}

#[test]
fn when_scrolling_the_content() {
    let mut ed = Editor::new(3, 10);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(&Key::Enter).unwrap();
    input_text(&mut ed, "jjj");

    assert_eq!(ed.window().content_view().height(), 2);
    assert_eq!(ed.window().content_view().line(0), "line 2");
    assert_eq!(ed.window().content_view().line(1), "line 3");
}
