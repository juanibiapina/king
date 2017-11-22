extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

fn editor() -> Editor {
    let mut ed = Editor::new(10, 10);
    ed.handle_key(Key::Char('i')).unwrap();
    return ed;
}

fn input_text(ed: &mut Editor, text: &str) {
    for c in text.chars() {
        ed.handle_key(Key::Char(c)).unwrap();
    }
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
fn entering_wide_chars_moves_the_cursor() {
    let mut ed = editor();

    input_text(&mut ed, "123😀abc");

    assert_eq!(ed.get_cursor(), (0, 8));
}
