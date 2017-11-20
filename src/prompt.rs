extern crate ncurses;

use std::char;

use self::ncurses as nc;

use ui;
use editor::Editor;
use mode::Mode;

pub struct Prompt {
    pub text: String,
    pub error: Option<String>,
    pub message: Option<String>,
    pub nwindow: nc::WINDOW,
}

pub fn delete_char(editor: &mut Editor) {
    editor.prompt.text.pop();

    if editor.prompt.text.is_empty() {
        end(editor);
    }
}

fn end(editor: &mut Editor) {
    editor.mode = Mode::Normal;
    editor.prompt.clear();
}

impl Prompt {
    pub fn new(y: i32) -> Prompt {
        let max_x = ui::getmaxx();
        let nwindow = ui::newwin(1, max_x, y, 0);

        Prompt {
            text: "".to_owned(),
            error: None,
            message: None,
            nwindow: nwindow,
        }
    }

    pub fn render_cursor(&self) {
        ui::wmove(self.nwindow, 0, self.text.len() as i32);
        ui::wnoutrefresh(self.nwindow);
    }

    pub fn start(&mut self, starting_char: char) {
        self.clear();
        self.add_char(starting_char);
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn clear(&mut self) {
        self.text = "".to_owned();
        self.message = None;
        self.error = None;
    }

    pub fn display_error(&mut self, text: &str) {
        self.error = Some(text.to_owned());
    }

    pub fn display_message(&mut self, text: &str) {
        self.message = Some(text.to_owned());
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }
}
