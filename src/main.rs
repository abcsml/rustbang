mod base;
mod display;
mod ai;

mod gobang;
mod tic_tac_toe;

use crate::base::{Game, Player, Board, Step, Role};
use crate::display::{Display, tui};
use crate::tic_tac_toe::{TTTBoard, TTTStep};

fn main() {
    // let mut game = Game::new(TTTBoard::new(), [Role::Hum, Role::Com], TTTStep::new(3,3));
    let mut game = Game::new(
        TTTBoard::new(),
        [Role::Com, Role::Hum],
        TTTStep::new(3,3,Player(0))
    );

    game.tui_main();

    println!("hello");
}
