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

#[derive(PartialEq)]
pub enum GameType {
    /// 只会落子，在棋盘上的棋子只会增加，不会消失
    /// 例：五子棋（gobang）
    Put,
    /// 棋子移动，会吃掉对方棋子，只会减少，不会增加
    /// 例：中国象棋
    Move,
}

pub struct Game<B: Board<S>, S: Step> {
    pub state: GameState,
    pub board: B,
    pub players: [Role; 2],
    pub curr_player: Player,    // 此时还未放棋子，用下标表示
    pub hist_steps: Vec<S>,
    pub game_type: GameType,
}

impl<B: Board<S>,S: Step> Game<B,S> {
    /// 创建一局新游戏
    pub fn new(board: B, players: [Role; 2], game_type: GameType) -> Self {
        Game {
            state: GameState::Running,
            board,
            players,
            curr_player: Player(0),
            hist_steps: vec![],
            game_type,
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
    /// 这步棋属于谁
    fn who(&self) -> Player;

    fn new_put_step(pos: (u8, u8), p: Player) -> Self;

    fn new_move_step(from: (u8, u8), to: (u8, u8), p: Player) -> Self;
}
