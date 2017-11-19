extern crate ncurses;

use self::ncurses as nc;

use std::char;

pub enum Key {
    Enter,
    Esc,
    Backspace,
    Char(char),
}

pub fn read_key() -> Option<Key> {
    match nc::wget_wch(nc::stdscr()) {
        Some(nc::WchResult::Char(ic)) => {
            match ic {
                27 => Some(Key::Esc),
                13 => Some(Key::Enter),
                127 => Some(Key::Backspace),
                ic => Some(Key::Char(parse_char(ic))),
            }
        },
        Some(nc::WchResult::KeyCode(_)) => None,
        None => None,
    }
}

fn parse_char(ic: u32) -> char {
    char::from_u32(ic).unwrap()
}
