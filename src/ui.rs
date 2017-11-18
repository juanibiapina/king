extern crate ncurses;

use self::ncurses as nc;

use key::Key;

pub fn init() {
    nc::initscr();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();
    nc::nonl();
}

pub fn finish() {
    nc::endwin();
}

pub fn get_key() ->  Option<Key> {
    match nc::wget_wch(nc::stdscr()) {
        Some(nc::WchResult::KeyCode(i)) => Some(Key::Code(i)),
        Some(nc::WchResult::Char(ic)) => Some(Key::Char(ic)),
        None => None,
    }
}

pub fn addstr(s: &str) {
    nc::addstr(s);
}

pub fn waddstr(w: nc::WINDOW, s: &str) {
    nc::waddstr(w, s);
}

pub fn addch(c: u32) {
    nc::addch(c);
}

pub fn mvaddch(y: i32, x: i32, c: u32) {
    nc::mvaddch(y, x, c);
}

pub fn getcury() -> i32 {
    nc::getcury(nc::stdscr())
}

pub fn getcurx() -> i32 {
    nc::getcurx(nc::stdscr())
}

pub fn getmaxy() -> i32 {
    nc::getmaxy(nc::stdscr())
}

pub fn getmaxx() -> i32 {
    nc::getmaxx(nc::stdscr())
}

pub fn wmove(w: nc::WINDOW, y: i32, x: i32) {
    nc::wmove(w, y, x);
}

pub fn mv(y: i32, x: i32) {
    nc::mv(y, x);
}

pub fn clrtoeol() {
    nc::clrtoeol();
}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> nc::WINDOW {
    nc::newwin(lines, cols, y, x)
}

pub fn wnoutrefresh(w: nc::WINDOW) {
    nc::wnoutrefresh(w);
}

pub fn werase(w: nc::WINDOW) {
    nc::werase(w);
}

pub fn doupdate() {
    nc::doupdate();
}
