use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

use error::{Error, Result};

pub type SharedBuffer = Rc<RefCell<Buffer>>;

pub struct Buffer {
    name: Option<String>,
    pub contents: Vec<String>,
}

pub fn create_buffer() -> SharedBuffer {
    Rc::new(RefCell::new(Buffer::new()))
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            name: None,
            contents: vec![String::new()],
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.name.is_none() && self.contents.len() == 1 && self.contents[0].is_empty()
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match reader.lines().collect::<io::Result<Vec<_>>>() {
                    Ok(lines) => {
                        self.name = Some(filename.to_owned());
                        if lines.len() == 0 {
                            self.contents = vec![String::new()];
                        } else {
                            self.contents = lines;
                        }

                        Ok(())
                    },
                    Err(err) => Err(Error::IoError(err)),
                }
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        self.name = Some(filename.to_owned());
                        self.contents = vec![String::new()];

                        Ok(())
                    },
                    _ => Err(Error::IoError(err)),
                }
            },
        }
    }
}
