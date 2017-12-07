extern crate king;

use king::editor::Editor;
use king::input::Key;
use king::mode::Mode;

#[test]
fn enter_to_prompt_with_colon() {
    let mut ed = Editor::new(10, 10);

    ed.handle_key(Key::Char(':')).unwrap();

    assert_eq!(ed.mode(), Mode::Prompt);
}
