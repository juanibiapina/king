use std::fs::File;
use std::io::Read;

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

    pub fn load(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut self.contents).unwrap();
    }
}
