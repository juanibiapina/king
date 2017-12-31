use std::char;

use error::Result;
use unicode;

pub struct Prompt {
    command_text: String,
    pub error: Option<String>,
    pub message: Option<String>,
    pub pos_y: usize,
    pub cur_x: usize,
}

impl Prompt {
    pub fn new(y: usize) -> Prompt {
        Prompt {
            command_text: "".to_owned(),
            error: None,
            message: None,
            pos_y: y,
            cur_x: 0,
        }
    }

    pub fn text(&self) -> &str {
        match self.error {
            Some(ref text) => text,
            None => {
                match self.message {
                    Some(ref text) => text,
                    None => &self.command_text,
                }
            },
        }
    }

    pub fn command_text(&self) -> &str {
        &self.command_text
    }

    pub fn start(&mut self, starting_char: char) -> Result<()> {
        self.clear();

        self.add_char(starting_char)
    }

    pub fn clear(&mut self) {
        self.command_text = "".to_owned();
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

    pub fn delete_grapheme(&mut self) {
        if let Some(c) = self.command_text.pop() {
            self.cur_x -= unicode::width_char(c);
        }
    }

    pub fn add_char(&mut self, c: char) -> Result<()> {
        self.command_text.push(c);
        self.cur_x += unicode::width_char(c);

        Ok(())
    }
}
