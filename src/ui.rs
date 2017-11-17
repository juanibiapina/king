extern crate ncurses;

use self::ncurses::*;

use key::Key;

pub fn init() {
    initscr();
    keypad(stdscr(), true);
    noecho();
}

pub fn finish() {
    endwin();
}

pub fn get_key() ->  Option<Key> {
    match wget_wch(stdscr()) {
        Some(WchResult::KeyCode(i)) => Some(Key::Code(i)),
        Some(WchResult::Char(ic)) => Some(Key::Char(ic)),
        None => None,
    }
}
