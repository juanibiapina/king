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

pub fn getmaxy() -> usize {
    nc::getmaxy(nc::stdscr()) as usize
}

pub fn getmaxx() -> usize {
    nc::getmaxx(nc::stdscr()) as usize
}

pub fn finish() {
    nc::endwin();
}

pub fn render(ed: &Editor) {
    erase();

    render_prompt(ed);
    render_window(ed);

    let (cur_y, cur_x) = ed.cursor();

    render_cursor(cur_y, cur_x);

    nc::refresh();
}

fn render_cursor(y: usize, x: usize) {
    nc::mv(y as i32, x as i32);
}

fn render_prompt(ed: &Editor) {
    render_text(ed.prompt.text(), ed.prompt.pos_y, 0);
}

fn render_window(ed: &Editor) {
    let window = ed.window();
    let (height, width) = window.size();
    let content_view = window.content_view();

    let mut row = 0;
    loop {
        if row >= height {
            break;
        }

        if row >= content_view.height() {
            break;
        }

        let line = content_view.line(row);

        if unicode::width(line) <= width {
            render_text(line, row, 0);
        } else {
            render_text_clipped(line, row, 0, width);
        }

        row += 1;
    }

    while row < height {
        mv(row, 0);
        addstr("~");
        row += 1;
    }
}

fn render_text(text: &str, y: usize, x: usize) {
    let mut column = x;
    for (_, grapheme) in unicode::graphemes(text) {
        column += render_grapheme(grapheme, y, column);
    }
}

fn render_text_clipped(text: &str, y: usize, x: usize, width: usize) {
    let mut column = x;
    for (_, grapheme) in unicode::graphemes(text) {
        column += render_grapheme(grapheme, y, column);

        if column >= width {
            break;
        }
    }
}

fn render_grapheme(grapheme: &str, y: usize, x: usize) -> usize {
    mv(y, x);
    addstr(grapheme);

    unicode::width(grapheme)
}

fn addstr(s: &str) {
    nc::addstr(s);
}

fn mv(y: usize, x: usize) {
    check(nc::mv(y as i32, x as i32));
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
