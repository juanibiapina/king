extern crate king;

use king::editor::Editor;
use king::input;
use king::error::error_message;
use king::ui;

fn main() {
    ui::init();

    let mut editor = Editor::init(ui::getmaxy(), ui::getmaxx());

    while editor.running() {
        if let Some(key) = input::read_key() {
            if let Err(err) = editor.handle_key(key) {
                editor.display_error(&error_message(err));
            }
        }

        ui::render(&editor);
    }

    ui::finish();
}
