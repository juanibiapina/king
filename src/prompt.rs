use std::char;

use ui;
use key::Key;

pub struct Prompt {
    y: i32,
    text: Option<String>,
}

impl Prompt {
    pub fn new(y: i32) -> Prompt {
        Prompt {
            y: y,
            text: None,
        }
    }

    pub fn run(&mut self, starting_char: u32) -> Option<String> {
        ui::mvaddch(self.y, 0, starting_char);

        loop {
            let key = ui::get_key();
            match key {
                Some(Key::Code(_)) => continue,
                Some(Key::Char(ic)) => {
                    match ic {
                        10 => {
                            let text = self.text.clone();
                            self.clear();
                            return text;
                        },
                        ic => {
                            match self.text {
                                Some(ref mut text) => text.push(char::from_u32(ic).unwrap()),
                                None => self.text = Some(char::from_u32(ic).unwrap().to_string()),
                            }
                            ui::addch(ic)
                        },
                    }
                },
                None => continue,
            }
        }
    }

    pub fn clear(&mut self) {
        self.text = None;

        ui::mv(self.y, 0);
        ui::clrtoeol();
    }
}
