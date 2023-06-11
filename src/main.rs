mod board;
mod score;
mod ai;

mod tui;

use crate::board::TTTBoard;

fn main() {
    // println!("Hello, world!");
    // let mut board = TTTBoard::new();
    // board.set(board::Pos(0, 1), board::Role::Com);
    // // board.set(board::Pos(1, 2), board::Role::Com);
    // board.set(board::Pos(1, 0), board::Role::Hum);
    // // board.set(board::Pos(2, 0), board::Role::Hum);
    // // board.set(board::Pos(1, 1), board::Role::Hum);
    // println!("{}", board);
    // println!("{:?}", ai::AI::get_score_map(board, board::Role::Com, 30));

    tui::tui_main();
}
