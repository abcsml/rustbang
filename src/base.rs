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

pub struct Game<B,S> {
    pub state: GameState,
    pub board: B,
    pub curr_player: Player,    // 此时还未放棋子
    pub hist_steps: Vec<S>,
}

impl<B: Board<S>,S: Copy> Game<B,S> {
    pub fn new(board: B, first: Player, step: S) -> Self {
        Game {
            state: GameState::Running,
            board: board,
            curr_player: first,
            hist_steps: vec![step; 0],
        }
    }

    pub fn step(&mut self, s: S) -> Result<(), Error> {
        self.board.put(s)?;
        self.state = self.board.over();
        self.curr_player = self.curr_player.rev();
        self.hist_steps.push(s);
        Ok(())
    }
}

pub trait Board<T: Copy> {
    /// 执行此step，
    fn put(&mut self, step: T) -> Result<(), Error>;

    /// 复制并执行
    fn copy_put(&self, step: T) -> Result<Self, Error> where Self: Sized;

    /// 判断当前是否意味着结束
    fn over(&self) -> GameState;

    /// 返回下一局可能走法
    fn get_possible_steps(&self, player: Player) -> Vec<T>;
}

// pub trait Map {
//     type Output;
//     fn to_array(&self) -> Self::Output;
// }
