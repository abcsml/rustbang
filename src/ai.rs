use std::cmp::{max, min};
use crate::base::{Player, Board, GameState, Step};

// struct AI {}

pub trait AI<S> :Board<S> {
    fn score(&self) -> i16;
}

pub fn get_score<B: AI<S>,S: Step>(board: &B, step: S, player: Player, deep: u8) -> i16 {
    let new_board = board.copy_put(step).unwrap();

    if deep == 1 || new_board.over() != GameState::Running {
        return new_board.score();
    }

    let mut max_score = i16::MIN;
    let mut min_score = i16::MAX;
    for s in new_board.get_possible_steps(player.rev()) {
        let score = get_score(&new_board, s, player.rev(), deep-1);
        max_score = max(max_score, score);
        min_score = min(min_score, score);
    }
    
    match player {
        Player::Com => min_score,
        Player::Hum => max_score,
    }
}

pub fn get_best_step<B: AI<S>,S: Step>(board: &B, player: Player) -> Option<S> {
    let mut best = None;
    let mut score = match player {
        Player::Com => i16::MIN,
        Player::Hum => i16::MAX,
    };
    for s in board.get_possible_steps(player) {
        let ns = get_score(board, s, player, 30);
        // println!("step: {:?}: {}", s, ns);
        if match player {
            Player::Com => score < ns,
            Player::Hum => score > ns,
        } {
            score = ns;
            best = Some(s);
        }
    }
    best
}
