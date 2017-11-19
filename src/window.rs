extern crate ncurses;

use std::char;

use self::ncurses as nc;

use ui;
use buffer::SharedBuffer;

pub struct Window {
    buffer: SharedBuffer,
    nwindow: nc::WINDOW,
    scroll_pos: i32,
    cur_y: i32,
    cur_x: i32,
    height: i32,
    width: i32,
}

impl Window {
    pub fn new(height: i32, width: i32, buffer: SharedBuffer) -> Window {
        let nwindow = ui::newwin(height, width, 0, 0);

        Window {
            buffer: buffer,
            nwindow: nwindow,
            scroll_pos: 0,
            cur_y: 0,
            cur_x: 0,
            height: height,
            width: width,
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.buffer.borrow().is_fresh()
    }

    fn adjust_cursor(&mut self) {
        let contents = &self.buffer.borrow().contents;
        let contents_len = contents.len() as i32;

        if self.cur_y >= self.height && self.cur_y < contents_len - 1 {
            self.cur_y = self.height - 1;
            self.scroll_pos += 1;
        }

        if self.cur_y >= contents_len {
            self.cur_y = contents_len - 1;
        }

        if self.cur_y < 0 {
            self.cur_y = 0;

            if self.scroll_pos > 0 {
                self.scroll_pos -= 1;
            }
        }

        if self.cur_y >= self.height {
            self.cur_y = self.height - 1;
        }

        let line_len = contents[self.cur_y as usize].len() as i32;

        if self.cur_x >= line_len {
            self.cur_x = line_len - 1;
        }

        if self.cur_x < 0 {
            self.cur_x = 0;
        }

        if self.cur_x >= self.width {
            self.cur_x = self.width - 1;
        }
    }

    pub fn move_cursor(&mut self, offset_y: i32, offset_x: i32) {
        self.cur_y += offset_y;
        self.cur_x += offset_x;

        self.adjust_cursor();
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
        let contents = &self.buffer.borrow().contents;
        let sliced = &contents[self.scroll_pos as usize .. ];
        for line in sliced {
            ui::wmove(self.nwindow, current_line, 0);
            ui::waddnstr(self.nwindow, line, max_x);

            current_line += 1;
            if current_line == max_y {
                break;
            }
        }

        while current_line < max_y {
            ui::wmove(self.nwindow, current_line, 0);
            ui::waddstr(self.nwindow, "~");
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

    pub fn add_char(&mut self, ic: u32) {
        let current_line = &mut self.buffer.borrow_mut().contents[(self.scroll_pos + self.cur_y) as usize];

        current_line.insert(self.cur_x as usize, char::from_u32(ic).unwrap());

        self.cur_x += 1;
    }
}
