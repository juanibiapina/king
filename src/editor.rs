use ui;
use key::Key;
use prompt::Prompt;

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
                        113 => break,
                        58 => {
                            let y = ui::getcury();
                            let x = ui::getcurx();

                            let command = self.prompt.run(58);

                            ui::mv(y,x);

                            match command {
                                Some(text) => ui::addstr(&text),
                                None => continue,
                            }
                        },
                        ic => {
                            ui::addstr(&ic.to_string())
                        },
                    }
                },
                None => continue,
            }
        }
    }
}
