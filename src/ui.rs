extern crate ncurses;

use self::ncurses as nc;

use editor::Editor;
use mode::Mode;
use unicode;

pub fn init() {
    nc::setlocale(nc::LcCategory::all, "");
    nc::initscr();
    check(nc::keypad(nc::stdscr(), true));
    check(nc::noecho());
    check(nc::raw());
    check(nc::nonl());
}

pub fn finish() {
    check(nc::endwin());
}

pub fn render(ed: &Editor) {
    render_window(ed);
    render_prompt(ed);

    match ed.mode {
        Mode::Normal => ed.window.render_cursor(),
        Mode::Insert => ed.window.render_cursor(),
        Mode::Prompt => ed.prompt.render_cursor(),
    }

    doupdate();
}

fn render_prompt(ed: &Editor) {
    werase(ed.prompt.nwindow);

    match ed.prompt.error {
        Some(ref text) => render_text(ed.prompt.nwindow, text),
        None => {
            match ed.prompt.message {
                Some(ref text) => render_text(ed.prompt.nwindow, text),
                None => render_text(ed.prompt.nwindow, &ed.prompt.text),
            };
        },
    };

    wnoutrefresh(ed.prompt.nwindow);
}

fn render_text(w: nc::WINDOW, text: &str) {
    let mut column = 0;
    for grapheme in unicode::graphemes(text, true) {
        let size = unicode::width(grapheme);
        wmove(w, 0, column);
        waddstr(w, grapheme);
        column += size as i32;
    }
}

fn render_window(ed: &Editor) {
    werase(ed.window.nwindow);

    let contents = &ed.window.buffer.borrow().contents;

    let mut row = 0;
    loop {
        if row >= ed.window.height {
            break;
        }

        let line_number = row + ed.window.scroll_pos;

        if line_number >= contents.len() as i32 {
            break;
        }

        let line = &contents[line_number as usize];

        let mut column = 0;
        for grapheme in unicode::graphemes(line, true) {
            let size = unicode::width(grapheme);

            if (column as usize) + size >= ed.window.width as usize {
                break;
            }

            wmove(ed.window.nwindow, row, column);
            waddstr(ed.window.nwindow, grapheme);
            column += size as i32;
        }

        row += 1;
    }

    while row < ed.window.height {
        wmove(ed.window.nwindow, row, 0);
        waddstr(ed.window.nwindow, "~");
        row += 1;
    }

    wnoutrefresh(ed.window.nwindow);
}

fn waddstr(w: nc::WINDOW, s: &str) {
    check(nc::waddstr(w, s));
}

pub fn getmaxy() -> i32 {
    nc::getmaxy(nc::stdscr())
}

pub fn getmaxx() -> i32 {
    nc::getmaxx(nc::stdscr())
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

fn werase(w: nc::WINDOW) {
    check(nc::werase(w));
}

fn doupdate() {
    check(nc::doupdate());
}

fn check(result: i32) {
    if result == nc::ERR {
        panic!("error");
    }
}
