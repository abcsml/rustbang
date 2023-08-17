mod util;

use std::{collections::HashSet, fmt::Debug};

use crate::{base::{Player, Step, Board, GameState}, display::{Display, log}, ai::AI};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct GoPos(u8, u8);

#[derive(PartialEq, Copy, Clone, Debug)]
enum GoPiece {
    P(Player),
    None,
}

#[derive(Debug, Clone)]
struct GoScores {
    five: u8,
    /// 两侧没有被封，有潜力成为五的
    /// one, two, therr, four
    non_block: [u8; 4],

    /// 一侧被封，有潜力成为五的
    /// block_two, block_three, block_four
    block: [u8; 3],
}

impl GoScores {
    const FIVE: i16 = 5000;

    /// one, two, three, four
    const NON_BLOCK: [i16; 4] = [0, 32, 256, 1024];
    /// block_two, block_three, block_four
    const BLOCK: [i16; 3]     = [2, 32, 256];
}

#[derive(Copy, Clone, Debug)]
pub struct GoStep {
    who: Player,
    pos: GoPos,
}

#[derive(Debug, Clone)]
pub struct GoBoard {
    pieces: [[GoPiece; 15]; 15],
    pieces_num: u8,
    /// 双方选手成绩，与Player下表对应
    scores: [GoScores; 2],
    // 下一步可能的棋子
    // next_pieces: HashSet<GoPos>,
    // next_pieces: [GoPiece; 200],
}

impl Step for GoStep {
    fn who(&self) -> Player {
        self.who
    }

    fn new_put_step(pos: (u8, u8), p: Player) -> Self {
        GoStep { who: p, pos: GoPos(pos.0, pos.1) }
    }

    fn new_move_step(from: (u8, u8), to: (u8, u8), p: Player) -> Self {
        panic!("Go: no move");
    }
}

impl GoStep {
    fn new(x: u8, y: u8, p: Player) -> Self {
        GoStep { who: p, pos: GoPos(x, y) }
    }
}

impl Board<GoStep> for GoBoard {
    fn new() -> Self {
        let mut start_pieces = HashSet::new();
        start_pieces.insert(GoPos(7, 7));

        GoBoard {
            pieces: [[GoPiece::None; 15]; 15],
            pieces_num: 0,
            scores: [GoScores::new(), GoScores::new()],
            // next_pieces: start_pieces
        }
    }

    fn put(&mut self, step: GoStep) -> bool {
        if self.can_put(&step) {
            self.update_scores(&step);
            *self.get_mut_piece(&step.pos) = GoPiece::P(step.who);
            self.pieces_num += 1;
            // self.update_next_pieces(&step.pos);
            return true;
        }
        false
    }

    fn over(&self) -> GameState {
        if self.scores[0].five == 1 {
            GameState::Over(crate::base::OutCome::Winer(Player(0)))
        } else if self.scores[1].five == 1 {
            GameState::Over(crate::base::OutCome::Winer(Player(1)))
        } else if self.pieces_num == 225 {
            // 平局判断需要加强
            GameState::Over(crate::base::OutCome::Draw)
        } else {
            GameState::Running
        }
    }
}

impl AI<GoStep> for GoBoard {
    /// 双方分别计分，一方减去另一方为最终得分
    fn score(&self, player: Player) -> i16 {
        let p0_score = self.scores[0].sum();
        let p1_score = self.scores[1].sum();

        if player.0 == 0 {
            p0_score - p1_score
        } else {
            p1_score - p0_score
        }
    }

    fn get_possible_steps(&self, player: Player, deep: u8) -> Vec<GoStep> {
        let mut v: Vec<(i16, GoStep)> = vec![];
        for i in 0..15 {
            for j in 0..15 {
                let step0 = GoStep::new(i, j, Player(0));
                let step1 = GoStep::new(i, j, Player(1));
                if self.get_piece(&step0.pos) == GoPiece::None {
                    if self.has_neighbor(&step0) || self.has_neighbor(&step1) {
                        let s0 = self.part_score(&step0);
                        let s1 = self.part_score(&step1);
                        v.push((s0+s1, GoStep::new(i, j, player)));
                    } else {
                        let s = -((i as i16 - 7).abs() + (j as i16 - 7).abs());
                        v.push((s, GoStep::new(i, j, player)));
                    }
                }
            }
        }

        // v.sort_by_key(|i|i.0);
        v.sort_by(|a,b|b.0.cmp(&a.0));
        // let b: Vec<GoStep> = v.iter().filter(|i|i.0>500).map(|i|i.1).collect();
        // 到最后两级时，只搜索高分的
        if deep < 5 {
            return v.iter().filter(|x|x.0>250&&x.0<512).map(|x|x.1).collect();
        }
        // 最后四级
        // if deep < 5 {
        //     return v.iter().filter(|x|x.0>500).map(|x|x.1).collect();
        // }
        // 最后六级
        // if deep < 7 {
        //     return v.iter().filter(|x|x.0>250).map(|x|x.1).collect();
        // }
        // 分级过滤
        for i in [1000, 512, 256, 0] {
            if v[0].0 > i {
                return v.iter().filter(|x|x.0>i).map(|x|x.1).collect();
            }
        }
        v.iter().filter(|x|x.0>-2).map(|x|x.1).collect()
    }
}

impl Display for GoBoard {
    fn to_array(&self) -> Vec<Vec<char>> {
        let mut arr = vec![vec![' '; 15]; 15];
        for i in 0..15 {
            for j in 0..15 {
                let p = self.get_piece(&GoPos(i, j));
                arr[i as usize][j as usize] = match p {
                    GoPiece::P(Player(0)) => '0',
                    GoPiece::P(Player(1)) => 'x',
                    _ => ' ',
                }
            }
        }
        arr
    }

    fn size(&self) -> (u8, u8) {
        (15, 15)
    }

    fn to_string(&self) -> String {
        format!("0:{}{:?}{:?} x:{}{:?}{:?}",
            self.scores[0].five,
            self.scores[0].non_block,
            self.scores[0].block,
            self.scores[1].five,
            self.scores[1].non_block,
            self.scores[1].block,
        )
    }
}
