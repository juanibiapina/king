extern crate ncurses;

use self::ncurses as nc;

use editor::Editor;
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
    nc::endwin();
}

pub fn render(ed: &Editor) {
    render_window(ed);
    render_prompt(ed);

    let (cur_y, cur_x) = ed.get_cursor();

    render_cursor(cur_y, cur_x);

    doupdate();
}

fn render_cursor(y: i32, x: i32) {
    nc::mv(y, x);
    nc::refresh();
}

fn render_prompt(ed: &Editor) {
    werase(ed.prompt.nwindow);

    match ed.prompt.error {
        Some(ref text) => render_text(ed.prompt.nwindow, text, 0, 0),
        None => {
            match ed.prompt.message {
                Some(ref text) => render_text(ed.prompt.nwindow, text, 0, 0),
                None => render_text(ed.prompt.nwindow, &ed.prompt.text, 0, 0),
            };
        },
    };

    wnoutrefresh(ed.prompt.nwindow);
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

        if unicode::width(line) <= ed.window.width as usize {
            render_text(ed.window.nwindow, line, row, 0);
        } else {
            render_text_clipped(ed.window.nwindow, line, row, 0, ed.window.width);
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

fn render_text(w: nc::WINDOW, text: &str, y: i32, x: i32) {
    let mut column = x;
    for grapheme in unicode::graphemes(text, true) {
        column += render_grapheme(w, grapheme, y, column) as i32;
    }
}

fn render_text_clipped(w: nc::WINDOW, text: &str, y: i32, x: i32, width: i32) {
    let mut column = x;
    for grapheme in unicode::graphemes(text, true) {
        column += render_grapheme(w, grapheme, y, column) as i32;

        if column >= width {
            break;
        }
    }
}

fn render_grapheme(w: nc::WINDOW, grapheme: &str, y: i32, x: i32) -> usize {
    wmove(w, y, x);
    waddstr(w, grapheme);

    unicode::width(grapheme)
}

fn waddstr(w: nc::WINDOW, s: &str) {
    nc::waddstr(w, s);
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
        finish();
        panic!("Ncurses error");
    }
}
