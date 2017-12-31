use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter, ErrorKind};

use error::{Error, Result};
use unicode;

pub struct Buffer {
    filename: Option<String>,
    contents: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer::new()
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            filename: None,
            contents: vec![String::new()],
        }
    }

    pub fn for_file(filename: &str) -> Result<Buffer> {
        Ok(Buffer {
            filename: Some(filename.to_owned()),
            contents: load_file(filename)?,
        })
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_ref().map(|s| &s[..])
    }

    pub fn add_line(&mut self, pos: usize) -> Result<()> {
        self.contents.insert(pos, "".to_owned());
        Ok(())
    }

    pub fn break_line(&mut self, y: usize, x: usize) -> Result<()> {
        let rest = self.contents[y].split_off(x);
        self.contents.insert((y + 1), rest);
        Ok(())
    }

    pub fn grapheme_at(&self, y: usize, x: usize) -> Option<(usize, String)> {
        let line = &self.contents[y];

        if x >= unicode::width(line) {
            return None;
        }

        let mut current_column = 0;
        for (offset, grapheme) in unicode::graphemes(line) {
            let size = unicode::width(grapheme);

            if current_column + size > x {
                return Some((offset, grapheme.to_owned()));
            }

            current_column += size;
        }

        None
    }

    pub fn join_lines(&mut self, n: usize) -> Result<()> {
        let line = self.contents.remove(n + 1);
        self.contents[n].push_str(&line);
        Ok(())
    }

    pub fn delete_char_at(&mut self, y: usize, x: usize) -> Result<Option<String>> {
        match self.grapheme_at(y, x) {
            Some((offset, grapheme)) => {
                self.contents[y].splice(offset..(offset+grapheme.len()), "");
                Ok(Some(grapheme))
            },
            None => Ok(None),
        }
    }

    pub fn line(&self, n: usize) -> &str {
        &self.contents[n]
    }

    pub fn line_mut(&mut self, n: usize) -> &mut String {
        &mut self.contents[n]
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn write(&self) -> Result<()> {
        match self.filename {
            Some(ref filename) => {
                match File::create(filename) {
                    Ok(file) => {
                        let mut writer = BufWriter::new(file);
                        for line in &self.contents {
                            match writer.write(line.as_bytes()) {
                                Ok(_) => {},
                                Err(err) => return Err(Error::IoError(err)),
                            };
                            match writer.write(b"\n") {
                                Ok(_) => {},
                                Err(err) => return Err(Error::IoError(err)),
                            };
                        }

                        Ok(())
                    },
                    Err(err) => Err(Error::IoError(err)),
                }
            },
            None => Err(Error::NoFileName),
        }
    }
}

fn load_file(filename: &str) -> Result<Vec<String>> {
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().collect::<io::Result<Vec<_>>>() {
                Ok(lines) => {
                    if lines.is_empty() {
                        Ok(vec![String::new()])
                    } else {
                        Ok(lines)
                    }
                },
                Err(err) => Err(Error::IoError(err)),
            }
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => Ok(vec![String::new()]),
                _ => Err(Error::IoError(err)),
            }
        },
    }
}
