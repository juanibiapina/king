extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

#[test]
fn i_enters_insert_mode() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char('i')).unwrap();

    assert_eq!(Mode::Insert, ed.mode);
}

#[test]
fn enter_to_prompt_with_colon() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char(':')).unwrap();

    assert_eq!(Mode::Prompt, ed.mode);
}
