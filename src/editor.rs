extern crate ncurses;

use self::ncurses::*;

pub struct Editor;

fn init() {
    initscr();
    keypad(stdscr(), true);
    noecho();
}

fn finish() {
    endwin();
}

impl Editor {
    pub fn new() -> Editor {
        Editor
    }

    pub fn run(&self) {
        init();

        loop {
            let ch = getch();
            match ch {
                113 => break,
                _ => continue,
            }
        }

        finish();
    }
}
