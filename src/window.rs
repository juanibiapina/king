use std::char;

use error::Result;
use buffer::Buffer;
use unicode;

pub struct Window {
    pub buffer: Buffer,
    pub scroll_pos: i32,
    pub cur_y: i32,
    pub cur_x: i32,
    height: i32,
    width: i32,
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

    pub fn adjust_cursor(&mut self) {
        let contents = &self.buffer.contents;
        let contents_len = contents.len() as i32;

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

        let line_len = unicode::width(&contents[(self.scroll_pos + self.cur_y) as usize]) as i32;

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

    pub fn move_cursor(&mut self, offset_y: i32, offset_x: i32) -> Result<()> {
        self.cur_y += offset_y;
        self.cur_x += offset_x;

        self.adjust_cursor();

        Ok(())
    }

    pub fn write(&mut self) -> Result<()> {
        self.buffer.write()
    }

    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
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
        let line_position = (self.scroll_pos + self.cur_y) as usize;
        let col_position = self.cur_x as usize;

        if self.cur_x == 0 {
           if self.cur_y > 0 {
               let prev_line = &self.buffer.contents[line_position-1].clone();
               self.cur_y -= 1;
               self.cur_x = (prev_line.len() - 1) as i32;
               return Ok(())
           } else {
               return Ok(())
           }
        }

        let current_line = &mut self.buffer.contents[line_position];
        let c = current_line.remove(col_position - 1);
        self.cur_x -= unicode::width_char(c) as i32;

        Ok(())
    }
}
