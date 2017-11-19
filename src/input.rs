extern crate ncurses;

use self::ncurses as nc;

use key::Key;

pub fn read_key() -> Option<Key> {
    match nc::wget_wch(nc::stdscr()) {
        Some(nc::WchResult::KeyCode(i)) => Some(Key::Code(i)),
        Some(nc::WchResult::Char(ic)) => Some(Key::Char(ic)),
        None => None,
    }
}

