extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

#[test]
fn starts_in_normal_mode() {
    let editor = Editor::new(10, 10);

    assert_eq!(Mode::Normal, editor.mode);
}

#[test]
fn i_switches_to_insert_mode() {
    let mut editor = Editor::new(10, 10);

    editor.handle_key(Key::Char('i')).unwrap();

    assert_eq!(Mode::Insert, editor.mode);
}
