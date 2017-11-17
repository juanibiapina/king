use ui;
use key::Key;

pub struct Editor;

impl Editor {
    pub fn new() -> Editor {
        Editor
    }

    pub fn run(&self) {
        ui::init();

        loop {
            let key = ui::get_key();
            match key {
                Some(Key::Code(_)) => continue,
                Some(Key::Char(ic)) => {
                    match ic {
                        113 => break,
                        _ => continue,
                    }
                },
                None => continue,
            }
        }

        ui::finish();
    }
}
