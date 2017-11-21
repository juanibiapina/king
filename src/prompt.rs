extern crate ncurses;

use std::char;

use self::ncurses as nc;

use ui;
use unicode;
use editor::Editor;
use mode::Mode;

pub struct Prompt {
    pub text: String,
    pub error: Option<String>,
    pub message: Option<String>,
    pub nwindow: nc::WINDOW,
    pub cur_x: usize,
}

pub fn delete_char(editor: &mut Editor) {
    if let Some(c) = editor.prompt.text.pop() {
        editor.prompt.cur_x -= unicode::width_char(c);
    }

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
            cur_x: 0,
        }
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
        self.cur_x = 0;
    }

    pub fn display_error(&mut self, text: &str) {
        self.error = Some(text.to_owned());
    }

    pub fn display_message(&mut self, text: &str) {
        self.message = Some(text.to_owned());
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
        self.cur_x += unicode::width_char(c);
    }
}
