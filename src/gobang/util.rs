use crate::{base::Player, display::log};

use super::{GoBoard, GoStep, GoScores, GoPos, GoPiece};
use Dir::*;

/// Direction, bool表示正负方向（偏右下角为正，偏左上角为负）
#[derive(PartialEq)]
enum Dir {
    Horiz(bool),
    Verti(bool),
    FSlash(bool),   // 斜杠   /
    BSlash(bool),   // 反斜杠 \
}

impl Dir {
    const DIRS: [Self; 8] = [
        Dir::Horiz(true), Dir::Horiz(false),
        Dir::Verti(true), Dir::Verti(false),
        Dir::FSlash(true), Dir::FSlash(false),
        Dir::BSlash(true), Dir::BSlash(false),
    ];

    fn get_pos_offset(&self) -> (i8, i8) {
        match *self {
            Dir::Horiz(true)    => ( 0,  1),
            Dir::Horiz(false)   => ( 0, -1),
            Dir::Verti(true)    => ( 1,  0),
            Dir::Verti(false)   => (-1,  0),
            Dir::FSlash(true)   => ( 1, -1),
            Dir::FSlash(false)  => (-1,  1),
            Dir::BSlash(true)   => ( 1,  1),
            Dir::BSlash(false)  => (-1, -1),
        }
    }
}

/// Condition
struct Cond {
    // dir: Dir,
    player: Player,
    num: u8,        // 有几个棋子（要么是对方，要么是己方）
    block: bool,    // 端点是否被挡
}

fn get_cond_by_dir(piece: &[[GoPiece; 15]; 15], pos: &GoPos, dir: &Dir) -> Cond {
    let mut player = Player(0);
    let mut num = 0;
    let mut block = false;

    let mut new_pos = pos.clone();
    if !new_pos.dir_add(dir) {
        return Cond {player: Player(0), num: 0, block: true};
    }

    if let GoPiece::P(p) = piece[new_pos.0 as usize][new_pos.1 as usize] {
        player = p;
        num += 1;
        for _ in 0..4 {
            if !new_pos.dir_add(dir) {  // 触碰边界
                block = true;
                break;
            }
            if let GoPiece::None = piece[new_pos.0 as usize][new_pos.1 as usize] {
                break;
            }
            if let GoPiece::P(x) = piece[new_pos.0 as usize][new_pos.1 as usize] {
                if x == player {
                    num += 1;
                } else {
                    block = true;
                    break;
                }
            }
        }
    }
    // log(format!("Cond: player:{}, num: {}, block: {}", player.0, num, block));
    Cond {player, num, block}
}

impl GoBoard {
    pub(super) fn can_put(&self, step: &GoStep) -> bool {
        step.pos.valid() && self.get_piece(&step.pos) == GoPiece::None
    }

    /// 假设pos有效，不再做冗余检查
    pub(super) fn get_piece(&self, pos: &GoPos) -> GoPiece {
        self.pieces[pos.0 as usize][pos.1 as usize]
    }

    /// 假设pos有效
    pub(super) fn get_mut_piece(&mut self, pos: &GoPos) -> &mut GoPiece {
        &mut self.pieces[pos.0 as usize][pos.1 as usize]
    }

    pub(super) fn update_scores(&mut self, step: &GoStep) {
        let score = &mut self.scores;
        let me_idx = step.who.0 as usize;
        let ot_idx = step.who.rev().0 as usize;

        // 四个维度
        for i in 0..4 {
            let fcond = get_cond_by_dir(&self.pieces, &step.pos, &Dir::DIRS[i*2]);
            let bcond = get_cond_by_dir(&self.pieces, &step.pos, &Dir::DIRS[i*2+1]);

            // 两边都有棋子
            if fcond.num > 0 && bcond.num > 0 {
                // 两边都是自己的棋
                if step.who == fcond.player && step.who == bcond.player {
                    score[me_idx].remove(fcond.num, fcond.block);
                    score[me_idx].remove(bcond.num, bcond.block);
                    score[me_idx].insert(fcond.num + 1 + bcond.num, fcond.block || bcond.block);
                // 一边是自己的棋
                } else if step.who == fcond.player || step.who == bcond.player {
                    let c = if step.who == fcond.player {&fcond} else {&bcond};

                    score[me_idx].remove(c.num, c.block);
                    if !c.block {
                        score[me_idx].insert(c.num + 1, true);
                    }
                // 都是对方棋子
                } else {
                    if fcond.block {
                        score[ot_idx].remove(fcond.num, fcond.block);
                    }
                    if bcond.block {
                        score[ot_idx].remove(bcond.num, bcond.block);
                    }
                }

            // 都没有棋子
            } else if fcond.num == 0 && bcond.num == 0 {
                score[me_idx].insert(1, false);
            
            // 只有一边有棋子
            } else {
                for c in [&fcond, &bcond] {
                    if c.num > 0 && c.player == step.who {
                        score[me_idx].remove(c.num, c.block);
                        score[me_idx].insert(c.num + 1, c.block);
                    } else if c.num > 0 && c.player != step.who {
                        score[ot_idx].remove(c.num, c.block);
                        if !c.block {
                            score[ot_idx].insert(c.num, true);
                        }
                    }
                }
            }
        }
        // log(format!("five: {}, block: {:?}, non-block:{:?}", self.scores[0].five,self.scores[0].block,self.scores[0].non_block));
        // log(format!("five: {}, block: {:?}, non-block:{:?}", self.scores[1].five,self.scores[1].block,self.scores[1].non_block));
    }

    fn get_pos_neighbor(&self, pos: &GoPos, piece: &GoPiece, level: u8) -> Vec<GoPos> {
        let mut v = vec![];
        for dir in Dir::DIRS {
            let mut p = pos.clone();
            p.dir_add(&dir);
            if self.get_piece(&p) == *piece {
                v.push(p);
            }
        }
        v
    }

    // pub(super) fn update_next_pieces(&mut self, pos: &GoPos) {
    //     if !self.next_pieces.remove(pos) {
    //         eprintln!("Error: GoBoard: pieces remove fault")
    //     }
    //     for i in self.get_pos_neighbor(pos, &GoPiece::None, 1) {
    //         self.next_pieces.insert(i);
    //     }
    // }

    pub(super) fn has_neighbor(&self, step: &GoStep) -> bool {
        for dir in Dir::DIRS {
            let mut p = step.pos.clone();
            p.dir_add(&dir);
            if self.get_piece(&p) == GoPiece::P(step.who) {
                return true;
            }
        }
        false
    }

    pub(super) fn part_score(&self, step: &GoStep) -> i16 {
        let scores = &self.scores;
        let me_idx = step.who.0 as usize;

        let mut score_sum = 0;
        // 四个维度
        for i in 0..4 {
            let fcond = get_cond_by_dir(&self.pieces, &step.pos, &Dir::DIRS[i*2]);
            let bcond = get_cond_by_dir(&self.pieces, &step.pos, &Dir::DIRS[i*2+1]);

            // 两边都有棋子
            if fcond.num > 0 && bcond.num > 0 {
                // 两边都是自己的棋
                if step.who == fcond.player && step.who == bcond.player {
                    if fcond.num + 1 + bcond.num == 5 {
                        return GoScores::FIVE;
                    }
                    // 没有全部阻塞
                    if !(fcond.block && bcond.block) {
                        score_sum += scores[me_idx]
                            .get(fcond.num+1+bcond.num, fcond.block || bcond.block);
                    }
                // 一边是自己的棋
                } else if step.who == fcond.player || step.who == bcond.player {
                    let c = if step.who == fcond.player {&fcond} else {&bcond};

                    if !c.block {
                        score_sum += scores[me_idx].get(c.num + 1, true);
                    }
                }
            
            // 只有一边有棋子
            } else {
                for c in [&fcond, &bcond] {
                    if c.num > 0 && c.player == step.who {
                        score_sum += scores[me_idx].get(c.num + 1, c.block);
                    }
                }
            }
        }
        score_sum
    }
}

impl GoPos {
    pub(super) fn valid(&self) -> bool {
        self.0 < 15 && self.1 < 15
    }

    fn dir_add(&mut self, dir: &Dir) -> bool {
        let offset = dir.get_pos_offset();
        // 简易情况
        if self.0 > 0 && self.0 < 14 && self.1 > 0 && self.1 < 14 {
            self.0 = ((self.0 as i8) + offset.0) as u8;
            self.1 = ((self.1 as i8) + offset.1) as u8;
            return true;
        }

        // 不合法情况过滤
        if self.0 == 0 {
            return !(*dir == Verti(false) || *dir == FSlash(false) || *dir == BSlash(false));
        } else if self.0 == 14 {
            return !(*dir == Verti(true) || *dir == FSlash(true) || *dir == BSlash(true));
        }

        if self.1 == 0 {
            return !(*dir == Horiz(false) || *dir == FSlash(true) || *dir == BSlash(false));
        } else if self.1 == 14 {
            return !(*dir == Horiz(true) || *dir == FSlash(false) || *dir == BSlash(true));
        }

        self.0 = ((self.0 as i8) + offset.0) as u8;
        self.1 = ((self.1 as i8) + offset.1) as u8;

        true
    }
}

impl GoScores {
    pub(super) fn new() -> Self {
        GoScores {
            five: 0,
            non_block: [0; 4],
            block: [0; 3],
        }
    }

    fn get(&self, flag: u8, block: bool) -> i16 {
        if flag >= 5 {
            return Self::FIVE;
        }

        let bi = flag as usize - 2;
        let nbi = flag as usize - 1;
        if block {
            Self::BLOCK[bi] * self.block[bi] as i16
        } else {
            Self::NON_BLOCK[nbi] * self.non_block[nbi] as i16
        }
    }

    fn insert(&mut self, flag: u8, block: bool) {
        // 过滤
        if flag == 1 {
            return;
        }

        if flag >= 5 {
            self.five += 1;
            return;
        }
        if block {
            self.block[flag as usize - 2] += 1;
        } else {
            self.non_block[flag as usize - 1] += 1;
        }
    }

    fn remove(&mut self, flag: u8, block: bool) -> bool {
        if (block && (flag < 2 || flag > 4)) || (!block && (flag < 1 || flag > 4)) {
            return false;
        }

        if block && self.block[flag as usize - 2] > 0 {
            self.block[flag as usize - 2] -= 1;
        } else if !block && self.non_block[flag as usize - 1] > 0 {
            self.non_block[flag as usize - 1] -= 1;
        } else {
            return false;
        }
        true
    }

    pub(super) fn sum(&self) -> i16 {
        let mut sum = (self.five as i16) * Self::FIVE;
        for i in 0..4 {
            sum += (self.non_block[i] as i16) * Self::NON_BLOCK[i];
        }
        for i in 0..3 {
            sum += (self.block[i] as i16) * Self::BLOCK[i];
        }
        sum
    }
}
