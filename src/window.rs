extern crate ncurses;

use std::rc::Rc;
use std::cell::RefCell;

use self::ncurses as nc;

use ui;
use buffer::Buffer;

pub struct Window {
    buffer: Rc<RefCell<Buffer>>,
    nwindow: nc::WINDOW,
}

impl Window {
    pub fn new(height: i32, width: i32, buffer: Rc<RefCell<Buffer>>) -> Window {
        let nwindow = ui::newwin(height, width, 0, 0);

        Window {
            buffer: buffer,
            nwindow: nwindow,
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.buffer.borrow().is_empty()
    }

    pub fn render(&self) {
        ui::werase(self.nwindow);
        ui::wmove(self.nwindow, 0, 0);
        ui::waddstr(self.nwindow, &self.buffer.borrow().contents);
        ui::mv(0, 0);
        ui::wrefresh(self.nwindow);
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Buffer>> {
        self.buffer.clone()
    }

    pub fn set_buffer(&mut self, buffer: Rc<RefCell<Buffer>>) {
        self.buffer = buffer;
    }
}
