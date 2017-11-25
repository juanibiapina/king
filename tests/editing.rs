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
    ed.handle_key(Key::Enter).unwrap();

    assert_eq!(Mode::Normal, ed.mode);
    assert_eq!(ed.prompt.message, Some("\"tests/fixtures/file_with_contents\"".to_owned()));
    assert_eq!(ed.window.buffer.contents[0], "line 1");
    assert_eq!(ed.window.buffer.contents[1], "line 2");
    assert_eq!(ed.window.buffer.contents[2], "line 3");
}
