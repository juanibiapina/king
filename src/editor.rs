use error::Result;
use input::Key;
use prompt::{self, Prompt};
use command::Command;
use buffer::{create_buffer, SharedBuffer};
use mode::Mode;
use window::Window;

pub struct Editor {
    pub mode: Mode,
    running: bool,
    pub prompt: Prompt,
    pub window: Window,
    buffers: Vec<SharedBuffer>,
    height: i32,
}

impl Editor {
    pub fn init(height: i32, width: i32) -> Editor {
        let buffer = create_buffer();
        let window = Window::new(height - 1, width, buffer.clone());
        let prompt = Prompt::new(height - 1);

        let editor = Editor {
            mode: Mode::Normal,
            prompt: prompt,
            window: window,
            running: true,
            buffers: vec![buffer.clone()],
            height: height,
        };

        return editor;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn display_error(&mut self, text: &str) {
        self.prompt.display_error(text);
    }

    pub fn get_cursor(&self) -> (i32, i32) {
        match self.mode {
            Mode::Normal => (self.window.cur_y, self.window.cur_x),
            Mode::Insert => (self.window.cur_y, self.window.cur_x),
            Mode::Prompt => (self.height - 1, self.prompt.cur_x as i32),
        }
    }

    pub fn handle_key(&mut self, key: Key) -> Result<()> {
        match self.mode {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
            Mode::Prompt => self.handle_key_prompt(key),
        }
    }

    fn handle_key_normal(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Char(c) => {
                match c {
                    ':' => self.switch_to_prompt(':'),
                    'i' => self.switch_to_insert(),
                    'h' => self.window.move_cursor(0, -1),
                    'j' => self.window.move_cursor(1, 0),
                    'k' => self.window.move_cursor(-1, 0),
                    'l' => self.window.move_cursor(0, 1),
                    _ => {},
                };
            },
            _ => {},
        };

        Ok(())
    }

    fn handle_key_prompt(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Char(c) => self.prompt.add_char(c),
            Key::Esc => self.cancel_prompt(),
            Key::Backspace => prompt::delete_char(self),
            Key::Enter => self.finish_prompt()?,
        };

        Ok(())
    }

    fn handle_key_insert(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Char(c) => self.window.add_char(c),
            Key::Esc => self.finish_insert(),
            Key::Backspace => self.window.delete_char(),
            _ => {},
        };

        Ok(())
    }

    fn finish_insert(&mut self) {
        self.switch_to_normal();
        self.window.adjust_cursor();
    }

    fn finish_prompt(&mut self) -> Result<()> {
        self.switch_to_normal();

        let text = self.prompt.get_text();
        self.prompt.clear();

        if text.is_empty() {
            return Ok(());
        }

        let command = Command::parse(&text)?;

        match command {
            Command::Quit => self.exit()?,
            Command::Write => self.write()?,
            Command::Edit(filename) => self.edit(&filename)?,
        };

        Ok(())
    }

    fn cancel_prompt(&mut self) {
        self.switch_to_normal();
        self.prompt.clear();
    }

    fn switch_to_prompt(&mut self, c: char) {
        self.mode = Mode::Prompt;
        self.prompt.start(c);
    }

    fn switch_to_insert(&mut self) {
        self.mode = Mode::Insert;
    }

    fn switch_to_normal(&mut self) {
        self.mode = Mode::Normal;
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;

        Ok(())
    }

    fn write(&mut self) -> Result<()> {
        let buffer = self.window.get_buffer();

        buffer.borrow_mut().write()?;

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

        self.prompt.display_message(&format!("\"{}\"", &filename));

        Ok(())
    }
}
