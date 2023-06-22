use std::fmt::Error;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    Hum,
    Com,
}

impl Player {
    pub fn rev(&self) -> Self {
        match *self {
            Player::Com => Player::Hum,
            Player::Hum => Player::Com,
        }
    }
}

#[derive(PartialEq)]
pub enum OutCome {
    Draw,
    Winer(Player),
}

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Over(OutCome),
}

pub struct Game<B: Board<S>, S: Step> {
    pub state: GameState,
    pub board: B,
    pub curr_player: Player,    // 此时还未放棋子
    pub hist_steps: Vec<S>,
}

impl<B: Board<S>,S: Step> Game<B,S> {
    pub fn new(board: B, first: Player, step: S) -> Self {
        Game {
            state: GameState::Running,
            board,
            curr_player: first,
            hist_steps: vec![step; 0],
        }
    }

    pub fn step(&mut self, s: S) -> Result<(), Error> {
        let _ = self.board.put(s);
        self.state = self.board.over();
        self.curr_player = self.curr_player.rev();
        self.hist_steps.push(s);
        Ok(())
    }
}

// pub enum StepResult {
//     Done,
//     None
// }

pub trait Board<S>: Clone {
    /// 执行此step，
    fn put(&mut self, step: S) -> bool;

    /// 复制并执行
    fn copy_put(&self, step: S) -> Option<Self> where Self: Sized {
        let mut new_board = self.clone();
        if new_board.put(step) {
            Some(new_board)
        } else {
            None
        }
    }

    /// 判断当前是否意味着结束
    fn over(&self) -> GameState;

    /// 返回下一局可能走法
    fn get_possible_steps(&self, player: Player) -> Vec<S>;
}

pub trait Step: Copy {
    /// 根据坐标创建一个Step，定义棋盘左上角为(0,0)
    fn new(x: u8, y: u8) -> Self;
}

// pub trait Map {
//     type Output;
//     fn to_array(&self) -> Self::Output;
// }
