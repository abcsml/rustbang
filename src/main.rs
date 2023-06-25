mod base;
mod display;
mod ai;

mod gobang;
mod tic_tac_toe;

use display::log;
use gobang::{GoBoard, GoStep};

use crate::base::{Game, Player, Board, Step, Role};
use crate::tic_tac_toe::{TTTBoard, TTTStep};

fn main() {
    // let mut game = Game::new(TTTBoard::new(), [Role::Hum, Role::Com], TTTStep::new(3,3));
    let mut game = Game::new(
        GoBoard::new(),
        [Role::Hum, Role::Com],
        GoStep::new(3,3,Player(0))
    );

    game.tui_main();
    // game.step(GoStep::new(7, 7, game.curr_player));
    // game.step(ai::get_next_best_step(&game.board, game.curr_player).unwrap());

    // println!("hello");
}
