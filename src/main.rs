mod base;
mod display;
mod ai;

mod tic_tac_toe;
mod gobang;
mod goT;

use base::*;
use display::log;

use tic_tac_toe::{TTTBoard, TTTStep};
use gobang::{GoBoard, GoStep};
use goT::{GoTBoard, GoTStep};

fn main() {
    // let mut game = Game::new(TTTBoard::new(), [Role::Hum, Role::Com], TTTStep::new(3,3));
    let mut game = Game::new(
        GoTBoard::new(),
        [Role::Hum, Role::Hum],
        GoTStep::new(5,5,Player(0))
    );

    game.tui_main();
    // game.step(GoStep::new(7, 7, game.curr_player));
    // game.step(ai::get_next_best_step(&game.board, game.curr_player).unwrap());

    // println!("hello");
}
