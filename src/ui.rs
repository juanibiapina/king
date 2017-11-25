extern crate ncurses;

use self::ncurses as nc;

use std::env;

use editor::Editor;
use unicode;

pub fn init() {
    env::set_var("ESCDELAY", "20");
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
    erase();

    render_prompt(ed);
    render_window(ed);

    let (cur_y, cur_x) = ed.get_cursor();

    render_cursor(cur_y, cur_x);

    nc::refresh();
}

fn render_cursor(y: i32, x: i32) {
    nc::mv(y, x);
}

fn render_prompt(ed: &Editor) {
    match ed.prompt.error {
        Some(ref text) => render_text(text, ed.prompt.pos_y, 0),
        None => {
            match ed.prompt.message {
                Some(ref text) => render_text(text, ed.prompt.pos_y, 0),
                None => render_text(&ed.prompt.text, ed.prompt.pos_y, 0),
            };
        },
    };
}

fn render_window(ed: &Editor) {
    let window = ed.window();
    let contents = &window.buffer.contents;

    let mut row = 0;
    loop {
        if row >= window.height {
            break;
        }

        let line_number = row + window.scroll_pos;

        if line_number >= contents.len() as i32 {
            break;
        }

        let line = &contents[line_number as usize];

        if unicode::width(line) <= window.width as usize {
            render_text(line, row, 0);
        } else {
            render_text_clipped(line, row, 0, window.width);
        }

        row += 1;
    }

    while row < window.height {
        mv(row, 0);
        addstr("~");
        row += 1;
    }
}

fn render_text(text: &str, y: i32, x: i32) {
    let mut column = x;
    for grapheme in unicode::graphemes(text, true) {
        column += render_grapheme(grapheme, y, column) as i32;
    }
}

fn render_text_clipped(text: &str, y: i32, x: i32, width: i32) {
    let mut column = x;
    for grapheme in unicode::graphemes(text, true) {
        column += render_grapheme(grapheme, y, column) as i32;

        if column >= width {
            break;
        }
    }
}

fn render_grapheme(grapheme: &str, y: i32, x: i32) -> usize {
    mv(y, x);
    addstr(grapheme);

    unicode::width(grapheme)
}

fn addstr(s: &str) {
    nc::addstr(s);
}

pub fn getmaxy() -> i32 {
    nc::getmaxy(nc::stdscr())
}

pub fn getmaxx() -> i32 {
    nc::getmaxx(nc::stdscr())
}

pub fn mv(y: i32, x: i32) {
    check(nc::mv(y, x));
}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> nc::WINDOW {
    nc::newwin(lines, cols, y, x)
}

fn erase() {
    check(nc::erase());
}

fn check(result: i32) {
    if result == nc::ERR {
        finish();
        panic!("Ncurses error");
    }
}
