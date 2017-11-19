use error::Result;
use ui;
use input::{self, Key};
use prompt::{self, Prompt};
use command::Command;
use error::error_message;
use buffer::{create_buffer, SharedBuffer};
use mode::Mode;
use window::Window;

pub struct Editor {
    pub mode: Mode,
    running: bool,
    pub prompt: Prompt,
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
            self.update();
            self.render();
        }
    }

    fn update(&mut self) {
        if let Some(key) = input::read_key() {
            if let Err(err) = self.handle_key(key) {
                self.prompt.display_error(&error_message(err));
            }
        }
    }

    fn render(&self) {
        self.window.render();
        self.prompt.render();

        match self.mode {
            Mode::Normal => self.window.render_cursor(),
            Mode::Insert => self.window.render_cursor(),
            Mode::Prompt => self.prompt.render_cursor(),
        }

        ui::doupdate();
    }

    fn handle_key(&mut self, key: Key) -> Result<()> {
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
            Key::Esc => self.cancel_prompt(),
            _ => {},
        };

        Ok(())
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
