use ui::Ui;
use key::Key;

pub struct Editor {
    ui: Ui,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            ui: Ui::new(),
        }
    }

    pub fn run(&self) {
        self.ui.init();

        loop {
            let key = self.ui.wget_wch();
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

        self.ui.finish();
    }
}
