pub mod tui;
mod util;

use std::fmt::Debug;

pub trait Display {
    // type Output;
    fn to_array(&self) -> Vec<Vec<char>>;

    fn size(&self) -> (u8, u8);
}

pub fn display<A: Debug>(array: A) {
    println!("{:?}", array);
}
