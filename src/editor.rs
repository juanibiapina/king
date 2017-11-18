use ui;
use key::Key;
use prompt::Prompt;
use command::Command;
use error::error_message;
use buffer::Buffer;

pub struct Editor {
    running: bool,
    prompt: Prompt,
    buffers: Vec<Buffer>,
    active_buffer: usize,
}

impl Editor {
    pub fn init() -> Editor {
        ui::init();

        let max_y = ui::getmaxy();

        Editor {
            prompt: Prompt::new(max_y - 1),
            running: true,
            buffers: vec![Buffer::new()],
            active_buffer: 0,
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

            self.render();
        }
    }

    fn render(&self) {
        ui::mv(0, 0);
        ui::addstr(&self.buffers[self.active_buffer].contents);
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Code(_) => return,
            Key::Char(ic) => {
                match ic {
                    58 => {
                        self.handle_prompt(58);
                    },
                    _ => return,
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
                            Command::Edit(filename) => self.edit(&filename),
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

    fn edit(&mut self, filename: &str) {
        if !self.buffers[self.active_buffer].is_empty() {
            self.buffers.push(Buffer::new());
            self.active_buffer = self.buffers.len() - 1;
        }

        self.buffers[self.active_buffer].load(filename);
    }
}
