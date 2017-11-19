extern crate king;

use king::editor::Editor;
use king::input;
use king::error::error_message;

fn main() {
    let mut editor = Editor::init();

    while editor.running() {
        if let Some(key) = input::read_key() {
            if let Err(err) = editor.handle_key(key) {
                editor.display_error(&error_message(err));
            }
        }

        editor.render();
    }

    editor.finish();
}
