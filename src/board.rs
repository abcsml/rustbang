use std::ops;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Role {
    Com,
    Hum,
    Empty,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            Role::Com => 'c',
            Role::Hum => 'h',
            Role::Empty => ' ',
        })
    }
}

impl Role {
    pub fn rev(&self) -> Self {
        if *self == Role::Com {
            return Role::Hum;
        }
        if *self == Role::Hum {
            return Role::Com;
        }
        Role::Empty
    }
    
    pub fn to_char(&self) -> char {
        match *self {
            Role::Com => 'c',
            Role::Hum => 'h',
            Role::Empty => ' ',
        }
    }
}

struct Board {}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pos(pub i8, pub i8);

impl ops::Add<[i8; 2]> for Pos{
    type Output = Self;

    fn add(self, other: [i8; 2]) -> Self {
        Self (
            self.0 + other[0],
            self.1 + other[1],
        )
    }
}

impl ops::Sub<[i8; 2]> for Pos{
    type Output = Self;

    fn sub(self, other: [i8; 2]) -> Self {
        Self (
            self.0 - other[0],
            self.1 - other[1],
        )
    }
}

impl Pos {
    pub fn valid(&self, min: i8, max: i8) -> bool {
        let p = self;
        (p.0 >= min && p.0 <= max) && (p.1 >= min && p.1 <= max)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct TTTBoard {
    pub pieces: [[Role; 3]; 3]
}

impl fmt::Display for TTTBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " {} │ {} │ {}",
            self.pieces[0][0], self.pieces[0][1], self.pieces[0][2])?;
        writeln!(f, "───┼───┼───")?;
        writeln!(f, " {} │ {} │ {}",
            self.pieces[1][0], self.pieces[1][1], self.pieces[1][2])?;
        writeln!(f, "───┼───┼───")?;
        writeln!(f, " {} │ {} │ {}",
            self.pieces[2][0], self.pieces[2][1], self.pieces[2][2])
    }
}

impl TTTBoard {
    pub fn new() -> TTTBoard {
        TTTBoard{
            pieces: [[Role::Empty; 3]; 3]
        }
    }

    pub fn size() -> u8 {
        3
    }

    pub fn set(&mut self, p: Pos, r: Role) {
        self.pieces[p.0 as usize][p.1 as usize] = r;
    }

    pub fn put(&mut self, p: Pos, r: Role) {
        if self.can_put(p) {
            self.pieces[p.0 as usize][p.1 as usize] = r;
        }
    }

    pub fn copy_set(&self, p: Pos, r: Role) -> Self {
        let mut b = self.clone();
        b.pieces[p.0 as usize][p.1 as usize] = r;
        b
    }

    pub fn get_poss(&self, r: Role) -> Vec<Pos> {
        let mut v = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.pieces[i as usize][j as usize] == r {
                    v.push(Pos(i, j));
                }
            }
        }
        v
    }

    pub fn get_empty_poss(&self) -> Vec<Pos> {
        self.get_poss(Role::Empty)
    }

    pub fn can_put(&self, p: Pos) -> bool {
        if (p.0 >= 0 && p.0 <= 2) && (p.1 >= 0 && p.1 <= 2) {
            return self.pieces[p.0 as usize][p.1 as usize] == Role::Empty;
        }
        false
    }

    pub fn over(&self) -> (bool, Role) {
        let ov = false;
        let ps = self.pieces;
        for r in [Role::Com, Role::Hum] {
            if ps[0][0] == r && ps[1][1] == r && ps[2][2] == r {
                return (true, r);
            }
            if ps[0][2] == r && ps[1][1] == r && ps[2][0] == r {
                return (true, r);
            }
            for i in 0..3 {
                if ps[i][0] == r && ps[i][1] == r && ps[i][2] == r {
                    return (true, r);
                }
            }
            for i in 0..3 {
                if ps[0][i] == r && ps[1][i] == r && ps[2][i] == r {
                    return (true, r);
                }
            }
        }
        // 平局
        for i in ps {
            for j in i {
                if j == Role::Empty {
                    return (false, Role::Empty);
                }
            }
        }
        (true, Role::Empty)
    }

    pub fn get_neighbours(&self, r: Role, p: Pos) -> Vec<Vec<Pos>> {
        // println!("pos: {:?}", p);
        let mut v = Vec::new();
        for offset in [[1,0], [0,1], [1,1], [1,-1]] {
            let mut vtmp = Vec::new();
            // 正方向
            let mut op = p;
            for _i in 0..2 {
                op = op + offset;
                // println!("pos op: {:?}", op);
                if op.valid(0, 2) && self.pieces[op.0 as usize][op.1 as usize] == r {
                    vtmp.push(op);
                } else {
                    break;
                }
            }
            // 反方向
            op = p;
            for _i in 0..2 {
                op = op - offset;
                // println!("neg op: {:?}", op);
                if op.valid(0, 2) && self.pieces[op.0 as usize][op.1 as usize] == r {
                    vtmp.push(op);
                } else {
                    break;
                }
            }
            if !vtmp.is_empty() {
                v.push(vtmp);
            }
        }
        v
    }
}
