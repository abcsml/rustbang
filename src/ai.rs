use std::cmp;

use crate::board::{TTTBoard, Role, Pos};
use crate::score::{TTTSCORE, ScoreBoard};

pub struct AI {}

impl AI {
    pub fn get_score_map(board: TTTBoard, r: Role, deep: u8) -> ScoreBoard {
        let mut map = ScoreBoard::new();

        for p in board.get_empty_poss() {
            let new_board = board.copy_set(p, r);
            if deep == 1 || new_board.over().0 {
                let score = TTTSCORE.score(&new_board, Role::Com) as i16;
                map.scores[p.0 as usize][p.1 as usize] = Some(score)
            } else {
                let score = Self::get_score_map(new_board, r.rev(), deep-1);
                if r == Role::Hum {
                    map.scores[p.0 as usize][p.1 as usize] = score.max();
                } else {
                    map.scores[p.0 as usize][p.1 as usize] = score.min();
                }
            }
        }
        map
    }

    pub fn put(board: &mut TTTBoard) {
        let map = AI::get_score_map(*board, Role::Com, 30);
        for p in board.get_empty_poss() {
            if map.scores[p.0 as usize][p.1 as usize].is_some() && map.max().is_some() {
                if map.scores[p.0 as usize][p.1 as usize].unwrap() == map.max().unwrap() {
                    board.put(p, Role::Com);
                    return;
                }
            }
        }
    }

    fn max(map: [[i16; 3]; 3]) -> i16 {
        let mut m = -64;
        for i in map {
            for j in i {
                m = cmp::max(j, m);
            }
        }
        m
    }

    fn min(map: [[i16; 3]; 3]) -> i16 {
        let mut m = 64;
        for i in map {
            for j in i {
                m = cmp::min(j, m);
            }
        }
        m
    }
}
