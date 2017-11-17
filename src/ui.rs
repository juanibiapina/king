extern crate ncurses;

use self::ncurses::*;

use key::Key;

pub struct Ui;

impl Ui {
    pub fn new() -> Ui {
        Ui
    }

    pub fn init(&self) {
        initscr();
        keypad(stdscr(), true);
        noecho();
    }

    pub fn finish(&self) {
        endwin();
    }

    pub fn wget_wch(&self) ->  Option<Key> {
        match wget_wch(stdscr()) {
            Some(WchResult::KeyCode(i)) => Some(Key::Code(i)),
            Some(WchResult::Char(ic)) => Some(Key::Char(ic)),
            None => None,
        }
    }
}
