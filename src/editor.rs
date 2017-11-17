use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;

pub struct Editor {
    prompt: Prompt,
    running: bool,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();

        Editor {
            prompt: Prompt::new(max_y - 1),
            running: true,
        }
    }

    pub fn finish(&self) {
        ui::finish();
    }

    pub fn run(&mut self) {
        while self.running {
            let key = ui::get_key();

            match key {
                Some(key) => self.handle_key(key),
                None => continue,
            }
        }
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Code(_) => return,
            Key::Char(ic) => {
                match ic {
                    58 => {
                        self.handle_prompt(58);
                    },
                    ic => {
                        // this is just for debugging
                        ui::addstr(&ic.to_string());
                        ui::addstr("|");
                    },
                }
            },
        }
    }

    fn handle_prompt(&mut self, ic: u32) {
        let y = ui::getcury();
        let x = ui::getcurx();

        let text = self.prompt.run(ic);

        match text {
            Some(text) => {
                let command = Command::parse(&text);

                match command {
                    Ok(command) => {
                        match command {
                            Command::Quit => self.exit(),
                        }
                    },
                    Err(err) => {
                        self.prompt.display_error(&error_message(err));
                    }
                }
            }
            None => return,
        }

        ui::mv(y,x);
    }

    fn exit(&mut self) {
        self.running = false;
    }
}
