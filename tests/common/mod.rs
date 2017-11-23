extern crate king;

use king::editor::Editor;
use king::input::Key;

pub fn input_text(ed: &mut Editor, text: &str) {
    for c in text.chars() {
        ed.handle_key(Key::Char(c)).unwrap();
    }
}
