use std::fmt::Error;
use crate::{base::{Player, Board, OutCome, GameState, Step}, ai::AI};
use crate::display::Display;

const SIZE: u8 = 3;

#[derive(PartialEq, Copy, Clone, Debug)]
struct TTTPos(u8, u8);

impl TTTPos {
    pub fn valid(&self) -> bool {
        if self.0 < SIZE && self.1 < SIZE {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct TTTPiece {
    belong: Player,
    pos: TTTPos,
}

#[derive(Copy, Clone, Debug)]
pub struct TTTStep {
    piece: TTTPiece
}

impl Step for TTTStep {
    // fn new(x: u8, y: u8, p: Player) -> Self {
    //     TTTStep { piece: TTTPiece { belong: p, pos: TTTPos(x, y) } }
    // }

    fn who(&self) -> Player {
        self.piece.belong
    }

    fn new_put_step(pos: (u8, u8), p: Player) -> Self {
        TTTStep { piece: TTTPiece { belong: p, pos: TTTPos(pos.0, pos.1) } }
    }

    fn new_move_step(from: (u8, u8), to: (u8, u8), p: Player) -> Self {
        panic!("TTT dont");
    }
}

#[derive(Clone)]
pub struct TTTBoard {
    pieces: Vec<TTTPiece>
}

impl TTTBoard {
    pub fn can_put(&self, step: &TTTStep) -> bool {
        if step.piece.pos.valid() {
            for p in &self.pieces {
                if p.pos == step.piece.pos {
                    return false;
                }
            }
            return true;
        }
        false
    }

    // fn get_all_steps(&self, player: Player) -> Vec<TTTStep> {
    //     let mut v = vec![];
    //     for i in 0..SIZE {
    //         for j in 0..SIZE {
    //             v.push(TTTStep {
    //                 piece: TTTPiece{belong: player, pos: TTTPos(i, j)}
    //             });
    //         }
    //     }
    //     v
    // }

    pub fn hand_put(&mut self, player: Player, x: u8, y: u8) -> bool {
        self.put(TTTStep { piece: TTTPiece { belong: player, pos: TTTPos(x, y) } })
    }
}

impl Board<TTTStep> for TTTBoard {
    fn new() -> Self {
        TTTBoard { pieces: vec![] }
    }

    fn put(&mut self, step: TTTStep) -> bool {
        if self.can_put(&step) {
            self.pieces.push(step.piece);
            return true;
        }
        false
    }

    fn over(&self) -> GameState {
        for pla in [Player(0), Player(1)] {
            let mut mat = [[false; SIZE as usize]; SIZE as usize];
            let mut flag = false;
            for pie in &self.pieces {
                if pie.belong == pla {
                    mat[pie.pos.0 as usize][pie.pos.1 as usize] = true;
                }
            }
            // println!("{:?}", mat);

            if mat[0][0] && mat[1][1] && mat[2][2] {
                flag = true;
            }
            if mat[0][2] && mat[1][1] && mat[2][0] {
                flag = true;
            }
            for i in 0..3 {
                if mat[i][0] && mat[i][1] && mat[i][2] {
                    flag = true;
                }
            }
            for i in 0..3 {
                if mat[0][i] && mat[1][i] && mat[2][i] {
                    flag = true;
                }
            }

            if flag {
                return GameState::Over(OutCome::Winer(pla));
            }
        }

        if self.pieces.len() == 9 {
            return GameState::Over(OutCome::Draw);
        }
        GameState::Running
    }
}

impl AI<TTTStep> for TTTBoard {
    fn score(&self, player: Player) -> i16 {
        match self.over() {
            GameState::Running => 0,
            GameState::Over(OutCome::Draw) => 0,
            GameState::Over(OutCome::Winer(p)) => {
                if p == player {32} else {-32}
            },
        }
    }

    fn get_possible_steps(&self, player: Player, deep: u8) -> Vec<TTTStep> {
        let mut v = vec![];
        for i in 0..SIZE {
            for j in 0..SIZE {
                let s = TTTStep {
                    piece: TTTPiece{belong: player, pos: TTTPos(i, j)}
                };
                if self.can_put(&s) {
                    v.push(s);
                }
            }
        }
        v
    }
}

impl Display for TTTBoard {
    fn to_array(&self) -> Vec<Vec<char>> {
        let mut arr = vec![vec![' '; 3]; 3];
        for p in &self.pieces {
            arr[p.pos.0 as usize][p.pos.1 as usize] =
            match p.belong.0 {
                0 => 'o',
                1 => 'x',
                _ => '?',
            }
        }
        arr
    }

    fn size(&self) -> (u8, u8) {
        (3, 3)
    }

    fn to_string(&self) -> String {
        String::new()
    }
}
