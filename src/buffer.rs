use std::fs::File;
use std::io::{Read, ErrorKind};

use error::{Error, Result};

pub struct Buffer {
    pub contents: String,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => return Err(Error::FileNotFound(filename.to_owned())),
                    _ => return Err(Error::IoError(err)),
                }
            },
        };

        match file.read_to_string(&mut self.contents) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::IoError(err)),
        }
    }
}
