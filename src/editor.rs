use std::rc::Rc;
use std::cell::RefCell;

use error::Result;
use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;
use buffer::Buffer;
use window::Window;

pub struct Editor {
    running: bool,
    prompt: Prompt,
    window: Window,
    buffers: Vec<Rc<RefCell<Buffer>>>,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();

        let buffer = Rc::new(RefCell::new(Buffer::new()));
        let window = Window::new(buffer.clone());

        Editor {
            prompt: Prompt::new(max_y - 1),
            window: window,
            running: true,
            buffers: vec![buffer.clone()],
        }
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
    }

    fn handle_key(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Code(_) => Ok(()),
            Key::Char(ic) => {
                match ic {
                    58 => self.handle_prompt(58),
                    _ => Ok(()),
                }
            },
        }
    }

    fn handle_prompt(&mut self, ic: u32) -> Result<()> {
        let y = ui::getcury();
        let x = ui::getcurx();

        let text = self.prompt.run(ic);

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
            None => return Ok(()),
        }

        ui::mv(y,x);

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
            buffer = Rc::new(RefCell::new(Buffer::new()));
            self.buffers.push(buffer.clone());
            self.window.set_buffer(buffer.clone());
        }

        buffer.borrow_mut().load(filename)?;

        Ok(())
    }
}
