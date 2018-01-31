mod board;
use self::board::*;

use std::fmt::{self, Display};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoMoreSpace,
    NoMoveOccurred,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    score: u32,
}

impl Game {
    pub fn new(size: usize) -> Game {
        let mut game = Game {
            board: Board::new(size),
            score: 0,
        };

        game.board.spawn_random().expect("spawn failed on board creation");

        game
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn is_over(&self) -> bool {
        self.board.is_full()
    }

    pub fn strafe(&mut self, d: Direction) -> Result<()> {
        let mut moved = false;

        for i in 0 .. self.board.size() {
            let s = match d {
                Direction::Up    => self.board.column_mut(i),
                Direction::Down  => self.board.column_mut(i).reverse(),
                Direction::Left  => self.board.row_mut(i),
                Direction::Right => self.board.row_mut(i).reverse(),
            };

            moved |= Game::squash(s)
        };

        if !moved {
            Err(Error::NoMoveOccurred)
        } else {
            self.board.spawn_random()
                .and(Some(()))
                .ok_or(Error::NoMoreSpace)
        }
    }

    fn squash(mut s: SliceMut) -> bool {
        let mut moved = false;

        'outer: for i in 0 .. s.len() {
            if s[i] == 0 {
                continue 'outer;
            }

            'inner: for j in 0 .. i {
                let obstacle_present = s[j+1 .. i].iter().any(|e| **e != 0);

                if !obstacle_present && (s[j] == 0 || s[i] == s[j]) {
                    moved = true;
                    s[j] += s[i];
                    s[i] = 0;
                    break 'inner;
                }
            }
        }

        moved
    }
}

#[derive(Debug)]
enum HBorder {
    Top,
    Inner,
    Bottom,
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.board.size();

        for i in 0 .. n {
            if i == 0 {
                writeln!(f, "{}", self.fmt_horiz_border(HBorder::Top))?;
            } else {
                writeln!(f, "{}", self.fmt_horiz_border(HBorder::Inner))?;
            }

            writeln!(f, "{}", self.fmt_inner_row(i))?;
        }

        writeln!(f, "{}", self.fmt_horiz_border(HBorder::Bottom))
    }
}

impl Game {
    fn fmt_horiz_border(&self, level: HBorder) -> String {
        let mut s = String::new();
        let n = self.board.size();

        let (lr, cr, rr) = match level {
            HBorder::Top    => ('┌', '┬', '┐'),
            HBorder::Inner  => ('├', '┼', '┤'),
            HBorder::Bottom => ('└', '┴', '┘'),
        };

        s.push_str(&format!("{}──────", lr));
        for _ in 1 .. n {
            s.push_str(&format!("{}──────", cr));
        };
        s.push_str(&format!("{}", rr));

        s
    }

    fn fmt_inner_row(&self, i: usize) -> String {
        let mut s = String::new();
        let n = self.board.size();

        for _ in 0 .. n {
            s.push_str("│      ");
        };
        s.push_str("│\n");

        for j in 0 .. n {
            s.push_str(&format!("│ {:^4} ", self.board[(i,j)]));
        };
        s.push_str("│\n");

        for _ in 0 .. n {
            s.push_str("│      ");
        };
        s.push_str("│");

        s
    }
}
