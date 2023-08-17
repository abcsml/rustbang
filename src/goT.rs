use crate::base::*;
use crate::ai::*;
use crate::display::*;

const SIZE:u8 = 4;

#[derive(PartialEq, Copy, Clone, Debug)]
enum GoTPiece {
    P(Player),
    None,
}

#[derive(Copy, Clone, Debug)]
struct GoTPos(u8, u8);

#[derive(Debug, Copy, Clone)]
pub struct GoTStep {
    who: Player,
    from: GoTPos,
    to: GoTPos,
}

#[derive(Clone)]
pub struct GoTBoard {
    pieces: [[GoTPiece; 4]; 4],
    pieces_num: [u8; 2],
}

impl Step for GoTStep {
    fn who(&self) -> Player {
        self.who
    }

    fn new_put_step(pos: (u8, u8), p: Player) -> Self {
        todo!()
    }

    fn new_move_step(from: (u8, u8), to: (u8, u8), p: Player) -> Self {
        GoTStep { who: p, from: GoTPos(from.0, from.1), to: GoTPos(to.0, to.1) }
    }
}

// impl MoveStep for GoTStep {
//     fn new(fx: u8, fy: u8, tx: u8, ty: u8, p: Player) -> Self {
//         GoTStep { who: p, from: GoTPos(fx, fy), to: GoTPos(tx, ty) }
//     }
// }

impl Board<GoTStep> for GoTBoard {
    fn new() -> Self {
        let mut pieces = [[GoTPiece::None; 4]; 4];
        pieces[0] = [GoTPiece::P(Player(0)); 4];
        pieces[3] = [GoTPiece::P(Player(1)); 4];
        GoTBoard { pieces, pieces_num: [4; 2] }
    }

    fn put(&mut self, step: GoTStep) -> bool {
        if self.can_put(&step) {
            *self.get_mut_piece(&step.from) = GoTPiece::None;
            *self.get_mut_piece(&step.to) = GoTPiece::P(step.who);
            // 检测是否吃掉对方棋子
            self.update_eat(&step);
            return true;
        }
        false
    }

    fn over(&self) -> GameState {
        if self.pieces_num[0] <= 1 {
            GameState::Over(OutCome::Winer(Player(0)))
        } else if self.pieces_num[1] <= 1 {
            GameState::Over(OutCome::Winer(Player(1)))
        } else {
            GameState::Running
        }
    }
}

impl AI<GoTStep> for GoTBoard {
    fn score(&self, player: Player) -> i16 {
        todo!()
    }

    fn get_possible_steps(&self, player: Player, deep: u8) -> Vec<GoTStep> {
        todo!()
    }
}

impl Display for GoTBoard {
    fn to_array(&self) -> Vec<Vec<char>> {
        let mut arr = vec![vec![' '; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let p = self.get_piece(&GoTPos(i, j));
                arr[i as usize][j as usize] = match p {
                    GoTPiece::P(Player(0)) => '0',
                    GoTPiece::P(Player(1)) => 'x',
                    _ => ' ',
                }
            }
        }
        arr
    }

    fn size(&self) -> (u8, u8) {
        (4, 4)
    }

    fn to_string(&self) -> String {
        format!("\n{:?}\n", self.pieces)
    }
}

// 其他（辅助函数）

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl GoTPiece {
    fn is_none(&self) -> bool {
        *self == GoTPiece::None
    }
}

impl GoTPos {
    fn valid(&self) -> bool {
        self.0 < 4 && self.1 < 4
    }

    fn distance(&self, pos: &GoTPos) -> u8 {
        ((self.0 as i8 - pos.0 as i8).abs() + (self.1 as i8 - pos.1 as i8).abs()) as u8
    }

    fn near_by(&self, dir: &Dir, i: u8) -> Option<Self> {
        let mut p = self.clone();
        match dir {
            Dir::Up => {
                if self.0 < i {None} else {p.0 -= i;Some(p)}
            }
            Dir::Down => {
                p.0 += i;
                if p.valid() {Some(p)} else {None}
            }
            Dir::Left => {
                if self.1 < i {None} else {p.1 -= i;Some(p)}
            }
            Dir::Right => {
                p.1 += i;
                if p.valid() {Some(p)} else {None}
            }
        }
    }
}

impl GoTBoard {
    fn can_put(&self, step: &GoTStep) -> bool {
        step.from.valid() && self.get_piece(&step.from) == GoTPiece::P(step.who) &&
        step.to.valid() && self.get_piece(&step.to) == GoTPiece::None &&
        step.from.distance(&step.to) == 1
    }

    /// 假设pos有效
    fn get_piece(&self, pos: &GoTPos) -> GoTPiece {
        self.pieces[pos.0 as usize][pos.1 as usize]
    }

    /// 假设pos有效
    fn get_mut_piece(&mut self, pos: &GoTPos) -> &mut GoTPiece {
        &mut self.pieces[pos.0 as usize][pos.1 as usize]
    }

    /// 再封装一次near_by
    fn get_near(&mut self, pos: &GoTPos, dir: &Dir, i: u8) -> GoTPiece {
        match pos.near_by(dir, i) {
            Some(x) => self.get_piece(&x),
            None => GoTPiece::None,
        }
    }

    fn update_eat(&mut self, step: &GoTStep) {
        for dirs in [[Dir::Up, Dir::Down], [Dir::Left, Dir::Right]] {
            let mut eat_idx = [1, 1];
            // let mut f_eat_idx = 1;
            // let mut b_eat_idx = 1;

            for i in 0..dirs.len() {
                let p = step.to.near_by(&dirs[i], 1);
                if p.is_some() &&
                self.get_piece(&p.unwrap()) == GoTPiece::P(step.who) {
                    eat_idx[i] = 2;
                }
            }
            
            // 必须是两个单独棋子
            if eat_idx[0] + eat_idx[1] != 3 {
                return;
            }

            let eat_piece: Vec<GoTPiece> = (0..2).map(|i| self.get_near(&step.to, &dirs[i], eat_idx[i])).collect();

            // let f_piece = self.get_near(&step.to, &dirs[0], eat_idx[0]);
            // let b_piece = self.get_near(&step.to, &dirs[1], eat_idx[1]);

            // 如果两边都有棋子，不能吃掉
            if !eat_piece[0].is_none() && !eat_piece[1].is_none() {
                return;
            // 都没有棋子
            } else if eat_piece[0].is_none() && eat_piece[1].is_none() {
                return;
            // 一边有棋子
            } else {
                for i in 0..2 {
                    if eat_piece[i] == GoTPiece::P(step.who.rev()) &&
                    self.get_near(&step.to, &dirs[0], eat_idx[i]+1).is_none() {
                        // eat
                        *self.get_mut_piece(&step.to.near_by(&dirs[0], eat_idx[i]).unwrap()) =
                            GoTPiece::None;
                        self.pieces_num[step.who.rev().0 as usize] -= 1;
                    }
                }
            }
        }
    }
}
