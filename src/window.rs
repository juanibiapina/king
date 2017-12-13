use std::char;
use std::cmp::min;

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

    pub fn adjust_cursor(&mut self) {
        let contents_len = self.buffer.len() as i32;

        if self.cur_y >= self.height {
            self.cur_y = self.height - 1;

            if self.cur_y + self.scroll_pos < contents_len - 1 {
                self.scroll_pos += 1;
            }
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

        let line = self.buffer.line(self.scroll_pos + self.cur_y);
        let line_len = unicode::width(self.buffer.line(self.scroll_pos + self.cur_y)) as i32;

        if self.cur_x >= line_len {
            self.cur_x = line_len - 1;
        }

        if self.cur_x < 0 {
            self.cur_x = 0;
        }

        if self.cur_x >= self.width {
            self.cur_x = self.width - 1;
        }

        if self.cur_x > 0 {
            let mut current_column = 0;
            for grapheme in unicode::graphemes(line, true) {
                let size = unicode::width(grapheme) as i32;

                if current_column + size < self.cur_x {
                    current_column += size;
                    continue;
                }

                if current_column + size == self.cur_x {
                    break;
                }

                if current_column + size > self.cur_x {
                    self.cur_x = current_column;
                    break;
                }
            }
        }
    }

    pub fn move_cursor(&mut self, offset_y: i32, offset_x: i32) -> Result<()> {
        self.cur_y += offset_y;
        self.cur_x += offset_x;
        self.adjust_cursor();

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
        let current_line = &mut self.buffer.contents[(self.scroll_pos + self.cur_y) as usize];

        if (self.cur_x as usize) >= unicode::width(&current_line) {
            current_line.push(c);
        } else {
            let byte_pos = unicode::byte_index_for_grapheme_index(&current_line, self.cur_x as usize);

            current_line.insert(byte_pos, c);
        }

        self.cur_x += unicode::width_char(c) as i32;

        Ok(())
    }

    pub fn delete_char(&mut self) -> Result<()> {
        let line_position = self.scroll_pos + self.cur_y;
        let col_position = self.cur_x as usize;

        if self.cur_x == 0 {
           if self.cur_y > 0 {
               let prev_line = &self.buffer.line(line_position - 1);
               self.cur_y -= 1;
               self.cur_x = (prev_line.len() - 1) as i32;
               return Ok(())
           } else {
               return Ok(())
           }
        }

        let current_line = &mut self.buffer.contents[line_position as usize];
        let c = current_line.remove(col_position - 1);
        self.cur_x -= unicode::width_char(c) as i32;

        Ok(())
    }
}
