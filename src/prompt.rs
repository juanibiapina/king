extern crate ncurses;

use std::char;

use self::ncurses as nc;

use ui;

pub struct Prompt {
    text: Option<String>,
    error: Option<String>,
    nwindow: nc::WINDOW,
}

impl Prompt {
    pub fn new(y: i32) -> Prompt {
        let max_x = ui::getmaxx();
        let nwindow = ui::newwin(1, max_x, y, 0);

        Prompt {
            text: None,
            error: None,
            nwindow: nwindow,
        }
    }

    pub fn move_cursor(&self) {
        match self.text {
            Some(ref text) => ui::wmove(self.nwindow, 0, text.len() as i32),
            None => ui::wmove(self.nwindow, 0, 0),
        }
        ui::wnoutrefresh(self.nwindow);
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
        ui::werase(self.nwindow);
        ui::wmove(self.nwindow, 0, 0);

        match self.error {
            Some(ref text) => ui::waddnstr(self.nwindow, text, -1),
            None => {
                match self.text {
                    Some(ref text) => ui::waddnstr(self.nwindow, text, -1),
                    None => {},
                }
            },
        }

        ui::wnoutrefresh(self.nwindow);
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
