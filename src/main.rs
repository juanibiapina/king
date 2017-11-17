extern crate ncurses;

use ncurses::*;

fn init() {
    initscr();
    keypad(stdscr(), true);
    noecho();
}

fn finish() {
    endwin();
}

fn main() {
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
