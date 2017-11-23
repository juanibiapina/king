use std::collections::HashMap;

use input::Key;
use command::Command;

pub struct Mappings {
    mappings: HashMap<Key, Command>,
}

impl Mappings {
    pub fn new() -> Mappings {
        Mappings {
            mappings: HashMap::new(),
        }
    }

    pub fn get(&self, key: &Key) -> Option<&Command> {
        self.mappings.get(key)
    }

    pub fn insert(&mut self, key: Key, command: Command) {
        self.mappings.insert(key, command);
    }
}
