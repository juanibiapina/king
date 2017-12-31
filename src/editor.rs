use error::Result;
use input::Key;
use prompt::Prompt;
use command::Command;
use movement::Movement;
use buffer::Buffer;
use mode::Mode;
use window::Window;
use mappings::Mappings;

pub struct Editor {
    mode: Mode,
    running: bool,
    pub prompt: Prompt,
    window: Window,
    height: usize,
    width: usize,
    normal_mappings: Mappings,
    insert_mappings: Mappings,
    prompt_mappings: Mappings,
}

impl Editor {
    pub fn new(height: usize, width: usize) -> Editor {
        let window = Window::new(height - 1, width, Buffer::new());
        let prompt = Prompt::new(height - 1);

        let mut ed = Editor {
            mode: Mode::Normal,
            prompt: prompt,
            window: window,
            running: true,
            height: height,
            width: width,
            normal_mappings: Mappings::new(),
            insert_mappings: Mappings::new(),
            prompt_mappings: Mappings::new(),
        };

        ed.add_mapping(Mode::Normal, Key::Char(':'), Command::EnterPrompt(':'));
        ed.add_mapping(Mode::Normal, Key::Char('i'), Command::EnterInsert);
        ed.add_mapping(Mode::Normal, Key::Char('a'), Command::EnterInsertAfterCursor);
        ed.add_mapping(Mode::Normal, Key::Char('o'), Command::OpenLineAfter);
        ed.add_mapping(Mode::Normal, Key::Char('O'), Command::OpenLineBefore);
        ed.add_mapping(Mode::Normal, Key::Char('h'), Command::Movement(Movement::Left));
        ed.add_mapping(Mode::Normal, Key::Char('j'), Command::Movement(Movement::Down));
        ed.add_mapping(Mode::Normal, Key::Char('k'), Command::Movement(Movement::Up));
        ed.add_mapping(Mode::Normal, Key::Char('l'), Command::Movement(Movement::Right));

        ed.add_mapping(Mode::Insert, Key::Esc, Command::LeaveInsert);
        ed.add_mapping(Mode::Insert, Key::Backspace, Command::DeleteCharBeforeCursor);

        ed.add_mapping(Mode::Prompt, Key::Esc, Command::CancelPrompt);
        ed.add_mapping(Mode::Prompt, Key::Enter, Command::RunPrompt);
        ed.add_mapping(Mode::Prompt, Key::Backspace, Command::DeleteCharBeforeCursorInPrompt);

        ed
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn size(&self) -> (usize, usize) {
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
        self.prompt.display_message(text);
    }

    pub fn cursor(&self) -> (usize, usize) {
        match self.mode {
            Mode::Normal | Mode::Insert => self.window.cursor(),
            Mode::Prompt => (self.height - 1, self.prompt.cur_x),
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> Result<()> {
        match self.mode {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
            Mode::Prompt => self.handle_key_prompt(key),
        }
    }

    fn handle_key_normal(&mut self, key: &Key) -> Result<()> {
        match self.normal_mappings.get(key).cloned() {
            Some(ref command) => self.run_command(command),
            None => Ok(()),
        }
    }

    fn handle_key_prompt(&mut self, key: &Key) -> Result<()> {
        match self.prompt_mappings.get(key).cloned() {
            Some(ref command) => self.run_command(command),
            None => {
                match *key {
                    Key::Char(c) => self.prompt.add_char(c),
                    _ => Ok(()),
                }
            },
        }
    }

    fn handle_key_insert(&mut self, key: &Key) -> Result<()> {
        match self.insert_mappings.get(key).cloned() {
            Some(ref command) => self.run_command(command),
            None => {
                match *key {
                    Key::Enter => self.window.break_line(),
                    Key::Char(c) => self.window.add_char(c),
                    _ => Ok(()),
                }
            },
        }
    }

    fn leave_insert(&mut self) -> Result<()> {
        self.switch_to_normal();
        self.window.ensure_cursor_over_line();

        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        self.switch_to_normal();

        let text = self.prompt.command_text().to_owned();
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
            Command::Write => self.window.write(),
            Command::Edit(ref filename) => self.edit(filename),
            Command::EnterPrompt(c) => self.enter_prompt(c),
            Command::CancelPrompt => self.cancel_prompt(),
            Command::RunPrompt => self.run_prompt(),
            Command::EnterInsert => self.enter_insert(),
            Command::EnterInsertAfterCursor => self.enter_insert_after_cursor(),
            Command::OpenLineAfter => self.open_line_after(),
            Command::OpenLineBefore => self.open_line_before(),
            Command::LeaveInsert => self.leave_insert(),
            Command::DeleteCharBeforeCursor => self.window.delete_char(),
            Command::DeleteCharBeforeCursorInPrompt => self.delete_char_in_prompt(),
            Command::Movement(movement) => self.window.move_cursor(movement),
        }
    }

    fn delete_char_in_prompt(&mut self) -> Result<()> {
        self.prompt.delete_grapheme();

        if self.prompt.command_text().is_empty() {
            self.switch_to_normal();
        }

        Ok(())
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

    fn enter_insert_after_cursor(&mut self) -> Result<()> {
        self.mode = Mode::Insert;
        self.window.advance_cursor()?;

        Ok(())
    }

    fn open_line_after(&mut self) -> Result<()> {
        self.mode = Mode::Insert;
        self.window.add_line_below()?;
        self.window.set_cur_x(0);
        self.window.move_cursor(Movement::Down)?;

        Ok(())
    }

    fn open_line_before(&mut self) -> Result<()> {
        self.mode = Mode::Insert;
        self.window.add_line_above()?;
        self.window.set_cur_x(0);

        Ok(())
    }

    fn switch_to_normal(&mut self) {
        self.mode = Mode::Normal;
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;

        Ok(())
    }

    fn edit(&mut self, filename: &str) -> Result<()> {
        let buffer = Buffer::for_file(filename)?;

        self.window.set_buffer(buffer);

        self.prompt.display_message(&format!("\"{}\"", &filename));

        Ok(())
    }
}
