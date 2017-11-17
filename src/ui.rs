extern crate ncurses;

use self::ncurses::*;

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

    pub fn getch(&self) -> i32 {
        getch()
    }
}
