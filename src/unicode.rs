extern crate unicode_width;
extern crate unicode_segmentation;

use self::unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use self::unicode_segmentation::{UnicodeSegmentation, GraphemeIndices};

pub fn width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

pub fn width_char(c: char) -> usize {
    UnicodeWidthChar::width(c).unwrap_or(0)
}

pub fn graphemes(text: &str) -> GraphemeIndices {
    UnicodeSegmentation::grapheme_indices(text, true)
}

pub fn byte_index_for_grapheme_index(text: &str, index: usize) -> usize {
    match UnicodeSegmentation::grapheme_indices(text, true).nth(index) {
        Some((index,_)) => index,
        None => 0,
    }
}
