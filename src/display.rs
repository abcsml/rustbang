pub mod tui;
mod util;

use std::fmt::Debug;

use ::tui::text::{Span, Spans};

pub trait Display {
    // type Output;
    fn to_array(&self) -> Vec<Vec<char>>;

    fn size(&self) -> (u8, u8);

    fn to_string(&self) -> String;
}

pub fn display<A: Debug>(array: A) {
    println!("{:?}", array);
}

pub fn log(s: String) {
    unsafe { tui::tui_log(Spans::from(Span::raw(s))) };
}
