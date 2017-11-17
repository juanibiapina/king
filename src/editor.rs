use ui::Ui;

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
            let ch = self.ui.getch();
            match ch {
                113 => break,
                _ => continue,
            }
        }

        self.ui.finish();
    }
}
