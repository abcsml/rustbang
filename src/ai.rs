use std::cmp::{max, min};
use crate::base::{Player, Board, GameState, Step};

// struct AI {}

pub trait AI<S> :Board<S> {
    fn score(&self, player: Player) -> i16;
}

/// player指对谁来评分
pub fn get_score<B: AI<S>,S: Step>(board: &B, step: S, me: Player,
    deep: u8, alpha: i16, beta: i16) -> i16 {

    let player = step.who();
    let new_board = board.copy_put(step).unwrap();

    if deep == 1 || new_board.over() != GameState::Running {
        return new_board.score(me);
    }

    let mut max_score = i16::MIN;
    let mut min_score = i16::MAX;
    for s in new_board.get_possible_steps(player.rev()) {
        let score = get_score(&new_board, s, me, deep-1, max_score, min_score);
        max_score = max(max_score, score);
        min_score = min(min_score, score);

        // alpha，beta剪枝
        if player == me && score < alpha {
            break;
        } else if player != me && score > beta {
            break;
        }
    }
    
    if player == me {
        min_score
    } else {
        max_score
    }
}

pub fn get_next_best_step<B: AI<S>,S: Step>(board: &B, player: Player) -> Option<S> {
    let mut best = None;
    let mut score = i16::MIN;
    for s in board.get_possible_steps(player) {
        let ns = get_score(board, s, player, 30, i16::MIN, i16::MAX);
        // println!("step: {:?}: {}", s, ns);
        if score < ns {
            score = ns;
            best = Some(s);
        }
    }
    best
}
