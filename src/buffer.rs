use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter, ErrorKind};

use error::{Error, Result};
use unicode;

pub struct Buffer {
    filename: Option<String>,
    pub contents: Vec<String>,
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

    pub fn add_line(&mut self, pos: i32) -> Result<()> {
        self.contents.insert(pos as usize, "".to_owned());
        Ok(())
    }

    pub fn grapheme_at(&self, y: i32, x: i32) -> Option<String> {
        let line = &self.contents[y as usize];

        if x >= unicode::width(line) as i32 || x < 0 {
            return None;
        }

        let mut current_column = 0;
        for grapheme in unicode::graphemes(line, true) {
            let size = unicode::width(grapheme) as i32;

            if current_column + size > x {
                return Some(grapheme.to_owned());
            }

            current_column += size;
        }

        return None;
    }

    pub fn join_lines(&mut self, n: i32) -> Result<()> {
        let i = n as usize;
        let line = self.contents.remove(i + 1);
        self.contents[i].push_str(&line);
        Ok(())
    }

    pub fn line(&self, n: i32) -> &str {
        &self.contents[n as usize]
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
                            match writer.write("\n".as_bytes()) {
                                Ok(_) => {},
                                Err(err) => return Err(Error::IoError(err)),
                            };
                        }

                        return Ok(());
                    },
                    Err(err) => return Err(Error::IoError(err)),
                }
            },
            None => return Err(Error::NoFileName),
        };
    }
}

fn load_file(filename: &str) -> Result<Vec<String>> {
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().collect::<io::Result<Vec<_>>>() {
                Ok(lines) => {
                    if lines.len() == 0 {
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
