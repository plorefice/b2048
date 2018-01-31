mod board;

pub use self::board::*;

use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Game {
    board: Board,
    score: u32,
}

impl Game {
    pub fn new(size: usize) -> Game {
        let game = Game {
            board: Board::new(size),
            score: 0,
        };

        game
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn over(&self) -> bool {
        self.board.is_full()
    }

    pub fn swipe(&mut self, d: Direction) -> Result<()> {
        self.board.swipe(d)
    }

}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
