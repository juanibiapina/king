extern crate ncurses;

use self::ncurses as nc;

use ui;
use buffer::SharedBuffer;

pub struct Window {
    buffer: SharedBuffer,
    nwindow: nc::WINDOW,
    cur_y: i32,
    cur_x: i32,
}

impl Window {
    pub fn new(height: i32, width: i32, buffer: SharedBuffer) -> Window {
        let nwindow = ui::newwin(height, width, 0, 0);

        Window {
            buffer: buffer,
            nwindow: nwindow,
            cur_y: 0,
            cur_x: 0,
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.buffer.borrow().is_fresh()
    }

    pub fn render_cursor(&self) {
        ui::wmove(self.nwindow, self.cur_y, self.cur_x);
        ui::wnoutrefresh(self.nwindow);
    }

    pub fn render(&self) {
        ui::werase(self.nwindow);

        let max_y = ui::wgetmaxy(self.nwindow);
        let max_x = ui::wgetmaxx(self.nwindow);

        let mut current_line = 0;
        for line in &self.buffer.borrow().contents {
            ui::wmove(self.nwindow, current_line, 0);
            ui::waddnstr(self.nwindow, line, max_x);

            current_line += 1;
            if current_line == max_y {
                break;
            }
        }

        while current_line < max_y {
            if current_line != 0 {
                ui::wmove(self.nwindow, current_line, 0);
                ui::waddstr(self.nwindow, "~");
            }
            current_line += 1;
        }

        ui::wnoutrefresh(self.nwindow);
    }

    pub fn get_buffer(&self) -> SharedBuffer {
        self.buffer.clone()
    }

    pub fn set_buffer(&mut self, buffer: SharedBuffer) {
        self.buffer = buffer;
    }
}
