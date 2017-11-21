// SIGTERM handling stuff
extern crate libc;
use libc::sighandler_t;
use libc::{c_int, c_void, SIGTERM};
use libc::signal;

use std::process::exit;

extern fn handler(_: c_int) {
    ui::finish();
    exit(143);
}

fn get_handler() -> sighandler_t {
    handler as extern fn(c_int) as *mut c_void as sighandler_t
}

// main
extern crate king;

use king::editor::Editor;
use king::input;
use king::error::error_message;
use king::ui;

fn main() {
    unsafe { signal(SIGTERM, get_handler()); }

    ui::init();

    let mut editor = Editor::new(ui::getmaxy(), ui::getmaxx());

    ui::render(&editor);

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
