use error::Result;
use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;
use buffer::{create_buffer, SharedBuffer};
use window::Window;

enum Mode {
    Normal,
    Prompt,
}

pub struct Editor {
    mode: Mode,
    running: bool,
    prompt: Prompt,
    window: Window,
    buffers: Vec<SharedBuffer>,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();
        let max_x = ui::getmaxx();

        let buffer = create_buffer();
        let window = Window::new(max_y - 1, max_x, buffer.clone());
        let prompt = Prompt::new(max_y - 1);

        let editor = Editor {
            mode: Mode::Normal,
            prompt: prompt,
            window: window,
            running: true,
            buffers: vec![buffer.clone()],
        };

        editor.render();

        return editor;
    }

    pub fn finish(&self) {
        ui::finish();
    }

    pub fn run(&mut self) {
        while self.running {
            let key = ui::get_key();

            match key {
                Some(key) => match self.handle_key(key) {
                    Ok(()) => {},
                    Err(err) => self.prompt.display_error(&error_message(err)),
                },
                None => {},
            };

            self.render();
        }
    }

    fn render(&self) {
        self.window.render();
        self.prompt.render();
        ui::doupdate();
    }

    fn handle_key(&mut self, key: Key) -> Result<()> {
        match self.mode {
            Mode::Normal => {
                match key {
                    Key::Code(_) => Ok(()),
                    Key::Char(ic) => {
                        match ic {
                            58 => self.switch_to_prompt(58),
                            _ => Ok(()),
                        }
                    },
                }
            },
            Mode::Prompt => {
                match key {
                    Key::Code(_) => Ok(()),
                    Key::Char(ic) => {
                        match ic {
                            13 => {
                                let text = self.prompt.get_text();
                                self.prompt.clear();
                                match text {
                                    Some(text) => {
                                        let command = Command::parse(&text);

                                        match command {
                                            Ok(command) => {
                                                match command {
                                                    Command::Quit => self.exit()?,
                                                    Command::Edit(filename) => self.edit(&filename)?,
                                                };
                                            },
                                            Err(err) => {
                                                self.prompt.display_error(&error_message(err));
                                            }
                                        }
                                    }
                                    None => {},
                                };

                                self.switch_to_normal()
                            },
                            ic => {
                                self.prompt.add_char(ic);
                                Ok(())
                            },
                        }
                    },
                }
            },
        }
    }

    fn switch_to_prompt(&mut self, ic: u32) -> Result<()> {
        self.mode = Mode::Prompt;

        self.prompt.start(ic);

        Ok(())
    }

    fn switch_to_normal(&mut self) -> Result<()> {
        self.mode = Mode::Normal;

        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;

        Ok(())
    }

    fn edit(&mut self, filename: &str) -> Result<()> {
        let buffer;

        if self.window.is_fresh() {
            buffer = self.window.get_buffer();
        } else {
            buffer = create_buffer();
            self.buffers.push(buffer.clone());
            self.window.set_buffer(buffer.clone());
        }

        buffer.borrow_mut().load(filename)?;

        Ok(())
    }
}
