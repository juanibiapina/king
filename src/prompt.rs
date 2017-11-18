use std::char;

use ui;

pub struct Prompt {
    y: i32,
    text: Option<String>,
    error: Option<String>,
}

impl Prompt {
    pub fn new(y: i32) -> Prompt {
        Prompt {
            y: y,
            text: None,
            error: None,
        }
    }

    pub fn start(&mut self, starting_char: u32) {
        self.clear();
        self.add_char(starting_char);
    }

    pub fn get_text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn clear(&mut self) {
        self.text = None;
        self.error = None;
    }

    pub fn render(&self) {
        ui::mv(self.y, 0);
        ui::clrtoeol();

        match self.error {
            Some(ref text) => ui::addstr(text),
            None => {
                match self.text {
                    Some(ref text) => ui::addstr(text),
                    None => {},
                }
            },
        }
    }

    pub fn display_error(&mut self, text: &str) {
        self.error = Some(text.to_owned());
    }

    pub fn add_char(&mut self, ic: u32) {
        match self.text {
            Some(ref mut text) => text.push(char::from_u32(ic).unwrap()),
            None => self.text = Some(char::from_u32(ic).unwrap().to_string()),
        }
    }
}
