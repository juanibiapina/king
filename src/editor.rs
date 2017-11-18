use error::Result;
use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;
use std::collections::HashMap;
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
    buffers: HashMap<String, SharedBuffer>,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();
        let max_x = ui::getmaxx();

        let buffer = create_buffer();
        let window = Window::new(max_y - 1, max_x, buffer.clone());
        let prompt = Prompt::new(max_y - 1);

        let mut buffers = HashMap::new();
        buffers.insert(String::new(), buffer);

        let editor = Editor {
            mode: Mode::Normal,
            prompt: prompt,
            window: window,
            running: true,
            buffers: buffers,
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

        match self.mode {
            Mode::Normal => {
                self.window.move_cursor();
            },
            Mode::Prompt => {
                self.prompt.move_cursor();
            },
        }

        ui::doupdate();
    }

    fn handle_key(&mut self, key: Key) -> Result<()> {
        match self.mode {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Prompt => self.handle_key_prompt(key),
        }
    }

    fn handle_key_normal(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Code(_) => Ok(()),
            Key::Char(ic) => {
                match ic {
                    58 => self.switch_to_prompt(58),
                    _ => Ok(()),
                }
            },
        }
    }

    fn handle_key_prompt(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Code(_) => {},
            Key::Char(ic) => {
                match ic {
                    13 => self.finish_prompt()?,
                    ic => self.prompt.add_char(ic),
                }
            },
        };

        Ok(())
    }

    fn finish_prompt(&mut self) -> Result<()> {
        self.switch_to_normal()?;

        let text = self.prompt.get_text();
        self.prompt.clear();

        match text {
            Some(text) => {
                let command = Command::parse(&text)?;

                match command {
                    Command::Quit => self.exit()?,
                    Command::Edit(filename) => self.edit(&filename)?,
                };
            }
            None => {},
        };

        Ok(())
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
        if let Some(buffer) = self.buffers.get(filename) {
            self.window.set_buffer(buffer.clone());
            return Ok(())
        }

        let buffer;
        if self.window.is_fresh() {
            buffer = self.window.get_buffer();
        } else {
            buffer = create_buffer();
            self.window.set_buffer(buffer.clone());
            self.buffers.insert(filename.to_string(), buffer);
        }

        buffer.borrow_mut().load(filename)?;

        Ok(())
    }
}
