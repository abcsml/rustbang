mod base;
mod display;
mod ai;

mod gobang;
mod tic_tac_toe;

use crate::base::{Game, Player, Board, Step};
use crate::display::{Display, tui};
use crate::tic_tac_toe::{TTTBoard, TTTStep};

fn main() {
    // let mut tem = tui::tui_init().unwrap();

    // // let mut board = TTTBoard::new();
    let mut game = Game::new(TTTBoard::new(), Player::Com, TTTStep::new(3,3));
    game.tui_main();
    // // game.step(ai::get_best_step(&game.board, game.curr_player).unwrap());

    // loop {
    //     // if let base::GameState::Over(_o) = game.state {
    //         // match o {
    //         //     base::OutCome::Draw
    //         // }
    //     // }
    //     tui::tui_draw(&mut tem, game.board.clone());

    //     let event = tui::tui_get_event();
    //     if let tui::TuiEvent::Exit = event {
    //         break;
    //     }

    //     if let tui::TuiEvent::GetPos((x, y)) = event {
    //         let step = Step::new(x as u8, y as u8);
    //         if game.state == base::GameState::Running {
    //             if game.curr_player == Player::Hum && game.board.can_put(&step) {
    //                 game.step(step);
    //             }
    //         }
    //     }

    //     if game.curr_player == Player::Com {
    //         game.step(ai::get_best_step(&game.board, game.curr_player).unwrap());
    //     }
    // }

    // tui::tui_exit(&mut tem);
    // let mut board = TTTBoard::new();
    // board.hand_put(Player::Com, 2, 0);
    // board.hand_put(Player::Com, 0, 1);
    // board.hand_put(Player::Com, 2, 2);
    // board.hand_put(Player::Hum, 0, 0);
    // board.hand_put(Player::Hum, 1, 0);
    // board.hand_put(Player::Hum, 2, 1);
    // let mut game = Game::new(board, Player::Com, TTTStep::new());
    // display::display(game.board.to_array());
    // let _ = game.step(ai::get_best_step(&game.board, game.curr_player).unwrap());
    // display::display(game.board.to_array());
    
    println!("hello");
}
