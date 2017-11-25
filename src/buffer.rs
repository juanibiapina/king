use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter, ErrorKind};

use error::{Error, Result};

pub struct Buffer {
    pub filename: Option<String>,
    pub contents: Vec<String>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            filename: None,
            contents: vec![String::new()],
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.filename.is_none() && self.contents.len() == 1 && self.contents[0].is_empty()
    }

    pub fn write(&mut self) -> Result<()> {
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

    pub fn load(&mut self, filename: &str) -> Result<()> {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match reader.lines().collect::<io::Result<Vec<_>>>() {
                    Ok(lines) => {
                        self.filename = Some(filename.to_owned());
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
                        self.filename = Some(filename.to_owned());
                        self.contents = vec![String::new()];

                        Ok(())
                    },
                    _ => Err(Error::IoError(err)),
                }
            },
        }
    }
}
