extern crate king;

use king::editor::Editor;
use king::input::Key;

mod common;
use common::input_text;

#[test]
fn vertical_scroll() {
    let mut ed = Editor::new(3, 3);

    input_text(&mut ed, ":edit tests/fixtures/file_with_contents");
    ed.handle_key(Key::Enter).unwrap();
    assert_eq!(ed.cursor(), (0, 0));
    assert_eq!(ed.window().content_view().line(0), "line 1");

    input_text(&mut ed, "j");
    assert_eq!(ed.cursor(), (1, 0));
    assert_eq!(ed.window().content_view().line(1), "line 2");

    input_text(&mut ed, "j");
    assert_eq!(ed.cursor(), (1, 0));
    assert_eq!(ed.window().content_view().line(1), "line 3");

    input_text(&mut ed, "j");
    assert_eq!(ed.cursor(), (1, 0));
    assert_eq!(ed.window().content_view().line(1), "line 3");

    input_text(&mut ed, "k");
    assert_eq!(ed.cursor(), (0, 0));
    assert_eq!(ed.window().content_view().line(0), "line 2");

    input_text(&mut ed, "k");
    assert_eq!(ed.cursor(), (0, 0));
    assert_eq!(ed.window().content_view().line(0), "line 1");
}
