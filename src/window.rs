use std::char;
use std::cmp::min;

use movement::Movement;
use error::Result;
use buffer::Buffer;
use unicode;

pub struct Window {
    buffer: Buffer,
    scroll_pos: i32,
    cur_y: i32,
    cur_x: i32,
    height: i32,
    width: i32,
}

pub struct ContentView<'a> {
    buffer: &'a Buffer,
    height: i32,
    vertical_offset: i32,
}

impl<'a> ContentView<'a> {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn line(&self, i: i32) -> &str {
        self.buffer.line(i + self.vertical_offset)
    }
}

impl Window {
    pub fn new(height: i32, width: i32, buffer: Buffer) -> Window {
        Window {
            buffer: buffer,
            scroll_pos: 0,
            cur_y: 0,
            cur_x: 0,
            height: height,
            width: width,
        }
    }

    pub fn size(&self) -> (i32, i32) {
        (self.height, self.width)
    }

    pub fn filename(&self) -> Option<&str> {
        self.buffer.filename()
    }

    pub fn cursor(&self) -> (i32, i32) {
        (self.cur_y, self.cur_x)
    }

    pub fn set_cur_x(&mut self, x: i32) {
        self.cur_x = x;
    }

    pub fn content_view(&self) -> ContentView {
        ContentView {
            buffer: &self.buffer,
            height: min(self.height, self.buffer.len() as i32),
            vertical_offset: self.scroll_pos,
        }
    }

    fn scroll_up(&mut self) {
        let contents_len = self.buffer.len() as i32;

        if self.cur_y + self.scroll_pos < contents_len - 1 {
            self.scroll_pos += 1;
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll_pos > 0 {
            self.scroll_pos -= 1;
        }
    }

    pub fn ensure_cursor_over_line(&mut self) {
        let line = self.buffer.line(self.cur_y + self.scroll_pos);
        let line_width = unicode::width(line) as i32;
        if self.cur_x >= line_width {
            if let Some((_, ref grapheme)) = self.buffer.grapheme_at(self.cur_y + self.scroll_pos, line_width - 1) {
                self.cur_x = line_width - (unicode::width(grapheme) as i32);
            }
        }
    }

    pub fn ensure_cursor_not_in_middle_of_widechar(&mut self) {
        if let Some((_, ref grapheme)) = self.buffer.grapheme_at(self.cur_y + self.scroll_pos, self.cur_x) {
            let size = unicode::width(grapheme) as i32;
            if size > 1 {
                self.cur_x -= size - 1;
            }
        }
    }

    pub fn move_cursor(&mut self, movement: Movement) -> Result<()> {
        match movement {
            Movement::Left => {
                self.cur_x -= 1;

                if self.cur_x < 0 {
                    self.cur_x = 0;
                }

                self.ensure_cursor_not_in_middle_of_widechar();
            },
            Movement::Right => {
                if let Some((_, ref grapheme)) = self.buffer.grapheme_at(self.cur_y + self.scroll_pos, self.cur_x) {
                    let size = unicode::width(grapheme) as i32;
                    let line_width = unicode::width(self.buffer.line(self.cur_y + self.scroll_pos)) as i32;
                    if self.cur_x + size < line_width {
                        self.cur_x += size;

                        if self.cur_x >= self.width {
                            self.cur_x = self.width - 1;
                        }
                    }
                }

            },
            Movement::Up => {
                self.cur_y -= 1;

                if self.cur_y < 0 {
                    self.cur_y = 0;

                    self.scroll_down();
                }

                self.ensure_cursor_over_line();
                self.ensure_cursor_not_in_middle_of_widechar();
            },
            Movement::Down => {
                self.cur_y += 1;

                if self.cur_y >= self.height {
                    self.cur_y = self.height - 1;

                    self.scroll_up();
                }

                let contents_len = self.buffer.len() as i32;

                if self.cur_y >= contents_len {
                    self.cur_y = contents_len - 1;
                }

                self.ensure_cursor_over_line();
                self.ensure_cursor_not_in_middle_of_widechar();
            },
        }

        Ok(())
    }

    pub fn advance_cursor(&mut self) -> Result<()> {
        let line = self.buffer.line(self.scroll_pos + self.cur_y);
        let line_len = unicode::width(line) as i32;

        if line_len > 0 {
            self.cur_x += 1;
        }

        Ok(())
    }

    pub fn add_line_below(&mut self) -> Result<()> {
        self.buffer.add_line(self.cur_y + 1)
    }

    pub fn add_line_above(&mut self) -> Result<()> {
        self.buffer.add_line(self.cur_y)
    }

    pub fn write(&mut self) -> Result<()> {
        self.buffer.write()
    }

    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
        self.cur_y = 0;
        self.cur_x = 0;
    }

    pub fn add_char(&mut self, c: char) -> Result<()> {
        if (self.cur_x as usize) >= unicode::width(self.buffer.line(self.scroll_pos + self.cur_y)) {
            let current_line = self.buffer.line_mut(self.scroll_pos + self.cur_y);
            current_line.push(c);
        } else {
            if let Some((offset, _)) = self.buffer.grapheme_at(self.scroll_pos + self.cur_y, self.cur_x) {
                let current_line = self.buffer.line_mut(self.scroll_pos + self.cur_y);
                current_line.insert(offset, c);
            }
        }

        self.cur_x += unicode::width_char(c) as i32;

        Ok(())
    }

    pub fn delete_char(&mut self) -> Result<()> {
        if self.cur_x == 0 {
            if self.cur_y == 0 {
                return Ok(())
            } else {
                let pos_x = unicode::width(self.buffer.line(self.cur_y - 1 + self.scroll_pos)) as i32;
                self.buffer.join_lines(self.cur_y - 1 + self.scroll_pos)?;

                self.cur_y -= 1;
                self.cur_x = pos_x;

                return Ok(())
            }
        }

        let line_position = self.scroll_pos + self.cur_y;

        if let Some(grapheme) = self.buffer.delete_char_at(line_position, self.cur_x - 1)? {
            self.cur_x -= unicode::width(&grapheme) as i32;
        }

        Ok(())
    }
}
