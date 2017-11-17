extern crate king;

use king::editor::Editor;

fn main() {
    let mut editor = Editor::init();

    editor.run();

    editor.finish();
}
