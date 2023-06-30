#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Role {
    Hum,
    Com,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Player(pub u8);

impl Player {
    pub fn rev(&self) -> Self {
        Player(match self.0 {
            0 => 1,
            1 => 0,
            _ => panic!("player error")
        })
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
    pub players: [Role; 2],
    pub curr_player: Player,    // 此时还未放棋子，用下标表示
    pub hist_steps: Vec<S>,
}

impl<B: Board<S>,S: Step> Game<B,S> {
    /// 创建一局新游戏
    pub fn new(board: B, players: [Role; 2], step: S) -> Self {
        Game {
            state: GameState::Running,
            board,
            players,
            curr_player: Player(0),
            hist_steps: vec![step; 0],
        }
    }

    /// 下一个执棋的玩家
    fn next_player(&self) -> Player {
        self.curr_player.rev()
    }

    ///只走一步
    pub fn step(&mut self, step: S) -> bool {
        if let GameState::Over(_) = self.state {
            return false;
        }
        let result = self.board.put(step);
        if result {
            self.state = self.board.over();
            self.curr_player = self.next_player();
            self.hist_steps.push(step);
        }
        result
    }
}

// pub enum StepResult {
//     Done,
//     None
// }

pub trait Board<S>: Clone {
    /// 创建一个新棋盘
    fn new() -> Self;

    /// 执行此step，
    fn put(&mut self, step: S) -> bool;

    /// 判断当前是否意味着结束
    fn over(&self) -> GameState;

    // 移入ai中
    // fn get_possible_steps(&self, player: Player) -> Vec<S>;

    /// 复制并执行
    fn copy_put(&self, step: S) -> Option<Self> where Self: Sized {
        let mut new_board = self.clone();
        if new_board.put(step) {
            Some(new_board)
        } else {
            None
        }
    }
}

pub trait Step: Copy {
    /// 根据坐标和玩家创建一个Step，定义棋盘左上角为(0,0)
    fn new(x: u8, y: u8, p: Player) -> Self;

    /// 这步棋属于谁
    fn who(&self) -> Player;
}

// pub trait Map {
//     type Output;
//     fn to_array(&self) -> Self::Output;
// }
