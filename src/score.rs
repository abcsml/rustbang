use crate::board::{Role, TTTBoard, Pos};

struct Score {}

#[derive(Debug)]
pub struct ScoreBoard {
    pub scores: [[Option<i16>; 3]; 3]
}

impl ScoreBoard {
    pub fn new() -> Self {
        ScoreBoard { scores: [[None; 3]; 3] }
    }
    
    pub fn max(&self) -> Option<i16> {
        let mut o = None;
            for i in self.scores {
                for j in i {
                    if j == None { continue; }
                    if o == None || o.as_ref().unwrap() < j.as_ref().unwrap() {
                        o = j;
                    }
                }
            }
        o
    }

    pub fn min(&self) -> Option<i16> {
        let mut o = None;
            for i in self.scores {
                for j in i {
                    if j == None { continue; }
                    if o == None || o.as_ref().unwrap() > j.as_ref().unwrap() {
                        o = j;
                    }
                }
            }
        o
    }
}

pub struct TTTScore {
    base: u8,
    center: u8,
    double: u8,
    triple: u8,
}

pub static TTTSCORE: TTTScore = TTTScore {
    base: 0,
    center: 2,
    double: 1,
    triple: 64,
};

impl TTTScore {
    pub fn score(&self, board: &TTTBoard, player: Role) -> i8 {
        let (o, w) = board.over();
        if o {
            if w == player {
                return 64;
            } else if w == player.rev() {
                return -64;
            } else {
                return 0;
            }
        }
        0
    }

    pub fn score_step(&self, my_role: Role, board: &TTTBoard, pos: Pos) -> u8 {
        let mut sc = self.base;
        if pos == Pos(1,1) {
            sc += self.center;
        }
        for line in board.get_neighbours(my_role, pos) {
            // print!("{:#?}\n", line);
            sc += match line.len() {
                0 => self.base,
                1 => self.double,
                2 => self.triple,
                _ => panic!("error"),
            }
        }
        sc
    }
}
