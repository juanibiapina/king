use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;

pub struct Editor {
    prompt: Prompt,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();

        Editor {
            prompt: Prompt::new(max_y - 1),
        }
    }

    pub fn finish(&self) {
        ui::finish();
    }

    pub fn run(&mut self) {
        loop {
            let key = ui::get_key();
            match key {
                Some(Key::Code(_)) => continue,
                Some(Key::Char(ic)) => {
                    match ic {
                        58 => {
                            let y = ui::getcury();
                            let x = ui::getcurx();

                            let text = self.prompt.run(58);

                            ui::mv(y,x);

                            match text {
                                Some(text) => {
                                    let command = Command::parse(&text);

                                    match command {
                                        Ok(command) => {
                                            match command {
                                                Command::Quit => break,
                                            }
                                        },
                                        Err(err) => {
                                            self.prompt.display_error(&error_message(err));
                                        }
                                    }
                                }
                                None => continue,
                            }
                        },
                        ic => {
                            ui::addstr(&ic.to_string());
                            ui::addstr("|");
                        },
                    }
                },
                None => continue,
            }
        }
    }
}
