extern crate king;

use king::editor::Editor;
use king::mode::Mode;

#[test]
fn starts_in_normal_mode() {
    let ed = Editor::new(10, 10);

    assert_eq!(ed.mode, Mode::Normal);
}

#[test]
fn starts_with_the_specified_dimentions() {
    let ed = Editor::new(25, 80);

    assert_eq!(ed.height, 25);
}

#[test]
fn starts_running() {
    let ed = Editor::new(10, 10);

    assert_eq!(ed.running(), true);
}

#[test]
fn initial_cursor_is_at_0_0() {
    let ed = Editor::new(10, 10);

    assert_eq!(ed.get_cursor(), (0, 0));
}

#[test]
fn starts_with_one_unnamed_buffer() {
    let ed = Editor::new(10, 10);

    assert_eq!(ed.buffer.borrow().filename, None);
}
