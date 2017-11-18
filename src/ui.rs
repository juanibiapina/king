extern crate ncurses;

use self::ncurses as nc;

use key::Key;

pub fn init() {
    nc::initscr();
    check(nc::keypad(nc::stdscr(), true));
    check(nc::noecho());
    check(nc::raw());
    check(nc::nonl());
}

pub fn finish() {
    check(nc::endwin());
}

pub fn get_key() ->  Option<Key> {
    match nc::wget_wch(nc::stdscr()) {
        Some(nc::WchResult::KeyCode(i)) => Some(Key::Code(i)),
        Some(nc::WchResult::Char(ic)) => Some(Key::Char(ic)),
        None => None,
    }
}

pub fn waddstr(w: nc::WINDOW, s: &str) {
    check(nc::waddstr(w, s));
}

pub fn waddnstr(w: nc::WINDOW, s: &str, n: i32) {
    check(nc::waddnstr(w, s, n));
}

pub fn getmaxy() -> i32 {
    nc::getmaxy(nc::stdscr())
}

pub fn getmaxx() -> i32 {
    nc::getmaxx(nc::stdscr())
}

pub fn wgetmaxy(w: nc::WINDOW) -> i32 {
    nc::getmaxy(w)
}

pub fn wgetmaxx(w: nc::WINDOW) -> i32 {
    nc::getmaxx(w)
}

pub fn wmove(w: nc::WINDOW, y: i32, x: i32) {
    check(nc::wmove(w, y, x));
}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> nc::WINDOW {
    nc::newwin(lines, cols, y, x)
}

pub fn wnoutrefresh(w: nc::WINDOW) {
    check(nc::wnoutrefresh(w));
}

pub fn werase(w: nc::WINDOW) {
    check(nc::werase(w));
}

pub fn doupdate() {
    check(nc::doupdate());
}

pub fn check(result: i32) {
    if result == nc::ERR {
        panic!("error");
    }
}
