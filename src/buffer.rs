use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

use error::{Error, Result};

pub struct Buffer {
    pub contents: Vec<String>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => return Err(Error::FileNotFound(filename.to_owned())),
                    _ => return Err(Error::IoError(err)),
                }
            },
        };

        let reader = BufReader::new(file);

        self.contents = match reader.lines().collect() {
            Ok(lines) => lines,
            Err(err) => return Err(Error::IoError(err)),
        };

        Ok(())
    }
}
