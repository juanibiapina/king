use error::Result;
use input::Key;
use prompt::{self, Prompt};
use command::Command;
use buffer::{create_buffer, SharedBuffer};
use mode::Mode;
use window::Window;
use mappings::Mappings;

pub struct Editor {
    pub mode: Mode,
    running: bool,
    pub prompt: Prompt,
    pub window: Window,
    pub buffer: SharedBuffer,
    height: i32,
    width: i32,
    normal_mappings: Mappings,
    insert_mappings: Mappings,
    prompt_mappings: Mappings,
}

impl Editor {
    pub fn new(height: i32, width: i32) -> Editor {
        let buffer = create_buffer();
        let window = Window::new(height - 1, width, buffer.clone());
        let prompt = Prompt::new(height - 1);

        let mut ed = Editor {
            mode: Mode::Normal,
            prompt: prompt,
            window: window,
            running: true,
            buffer: buffer.clone(),
            height: height,
            width: width,
            normal_mappings: Mappings::new(),
            insert_mappings: Mappings::new(),
            prompt_mappings: Mappings::new(),
        };

        ed.add_mapping(Mode::Normal, Key::Char(':'), Command::EnterPrompt(':'));
        ed.add_mapping(Mode::Normal, Key::Char('i'), Command::EnterInsert);
        ed.add_mapping(Mode::Normal, Key::Char('h'), Command::MoveCursorLeft);
        ed.add_mapping(Mode::Normal, Key::Char('j'), Command::MoveCursorDown);
        ed.add_mapping(Mode::Normal, Key::Char('k'), Command::MoveCursorUp);
        ed.add_mapping(Mode::Normal, Key::Char('l'), Command::MoveCursorRight);

        ed.add_mapping(Mode::Insert, Key::Esc, Command::LeaveInsert);
        ed.add_mapping(Mode::Insert, Key::Backspace, Command::DeleteCharBeforeCursor);

        ed.add_mapping(Mode::Prompt, Key::Esc, Command::CancelPrompt);
        ed.add_mapping(Mode::Prompt, Key::Enter, Command::RunPrompt);
        ed.add_mapping(Mode::Prompt, Key::Backspace, Command::DeleteCharBeforeCursorInPrompt);

        return ed;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> (i32, i32) {
        (self.height, self.width)
    }

    pub fn add_mapping(&mut self, mode: Mode, key: Key, command: Command) {
        match mode {
            Mode::Normal => self.normal_mappings.insert(key, command),
            Mode::Insert => self.insert_mappings.insert(key, command),
            Mode::Prompt => self.prompt_mappings.insert(key, command),
        }
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
        match self.normal_mappings.get(&key).cloned() {
            Some(ref command) => self.run_command(command),
            None => Ok(()),
        }
    }

    fn handle_key_prompt(&mut self, key: Key) -> Result<()> {
        match self.prompt_mappings.get(&key).cloned() {
            Some(ref command) => self.run_command(command),
            None => {
                match key {
                    Key::Char(c) => self.prompt.add_char(c),
                    _ => Ok(()),
                }
            },
        }
    }

    fn handle_key_insert(&mut self, key: Key) -> Result<()> {
        match self.insert_mappings.get(&key).cloned() {
            Some(ref command) => self.run_command(command),
            None => {
                match key {
                    Key::Char(c) => self.window.add_char(c),
                    _ => Ok(()),
                }
            },
        }
    }

    fn leave_insert(&mut self) -> Result<()> {
        self.switch_to_normal();
        self.window.adjust_cursor();

        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        self.switch_to_normal();

        let text = self.prompt.get_text();
        self.prompt.clear();

        if text.is_empty() {
            return Ok(());
        }

        let command = Command::parse(&text)?;

        self.run_command(&command)
    }

    fn run_command(&mut self, command: &Command) -> Result<()> {
        match *command {
            Command::Quit => self.exit(),
            Command::Write => self.write(),
            Command::Edit(ref filename) => self.edit(filename),
            Command::EnterPrompt(c) => self.enter_prompt(c),
            Command::CancelPrompt => self.cancel_prompt(),
            Command::RunPrompt => self.run_prompt(),
            Command::EnterInsert => self.enter_insert(),
            Command::LeaveInsert => self.leave_insert(),
            Command::DeleteCharBeforeCursor => self.window.delete_char(),
            Command::DeleteCharBeforeCursorInPrompt => prompt::delete_char(self),
            Command::MoveCursorLeft => self.window.move_cursor(0, -1),
            Command::MoveCursorRight => self.window.move_cursor(0, 1),
            Command::MoveCursorUp => self.window.move_cursor(-1, 0),
            Command::MoveCursorDown => self.window.move_cursor(1, 0),
        }
    }

    fn cancel_prompt(&mut self) -> Result<()> {
        self.switch_to_normal();
        self.prompt.clear();

        Ok(())
    }

    fn enter_prompt(&mut self, c: char) -> Result<()> {
        self.mode = Mode::Prompt;
        self.prompt.start(c)
    }

    fn enter_insert(&mut self) -> Result<()> {
        self.mode = Mode::Insert;

        Ok(())
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
        let buffer = create_buffer();
        buffer.borrow_mut().load(filename)?;

        self.window.set_buffer(buffer.clone());

        self.prompt.display_message(&format!("\"{}\"", &filename));

        Ok(())
    }
}
