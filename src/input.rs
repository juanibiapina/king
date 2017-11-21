extern crate ncurses;

use self::ncurses as nc;

use std::char;

pub enum Key {
    Enter,
    Esc,
    Backspace,
    Char(char),
    Unknown,
}

pub fn read_key() -> Option<Key> {
    match nc::wget_wch(nc::stdscr()) {
        Some(nc::WchResult::Char(ic)) => {
            match ic {
                27 => Some(Key::Esc),
                13 => Some(Key::Enter),
                127 => Some(Key::Backspace),
                ic => {
                    match parse_char(ic) {
                        Some(c) => Some(Key::Char(c)),
                        None => Some(Key::Unknown),
                    }
                }
            }
        },
        Some(nc::WchResult::KeyCode(_)) => None,
        None => None,
    }
}

fn parse_char(ic: u32) -> Option<char> {
    char::from_u32(ic)
}
