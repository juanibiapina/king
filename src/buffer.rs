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

    pub fn for_file(filename: &str) -> Result<Buffer> {
        Ok(Buffer {
            filename: Some(filename.to_owned()),
            contents: load_file(filename)?,
        })
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
