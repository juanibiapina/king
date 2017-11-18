use std::rc::Rc;
use std::cell::RefCell;

use buffer::Buffer;

pub struct Window {
    buffer: Rc<RefCell<Buffer>>,
}

impl Window {
    pub fn new(buffer: Rc<RefCell<Buffer>>) -> Window {
        Window {
            buffer: buffer,
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.buffer.borrow().is_empty()
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Buffer>> {
        self.buffer.clone()
    }

    pub fn set_buffer(&mut self, buffer: Rc<RefCell<Buffer>>) {
        self.buffer = buffer;
    }
}
