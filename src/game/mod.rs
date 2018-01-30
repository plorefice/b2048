use nalgebra::Matrix4;

use rand::{self, Rng};

use std::fmt::{self, Display};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoMoreSpace,
    NoMoveOccurred,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub struct Game {
    tiles: Matrix4<u32>,
    score: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            tiles: Matrix4::zeros(),
            score: 0,
        };

        game.gen_random_tile().unwrap();

        game
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn is_over(&self) -> bool {
        !self.tiles.iter().any(|&v| v == 0)
    }

    pub fn strafe(&mut self, d: Direction) -> Result<()> {
        use Direction::*;

        let moves = match d {
            d @ Up   | d @ Down  => (0..4).map(|i| self.vstrafe(i, d)).collect::<Vec<_>>(),
            d @ Left | d @ Right => (0..4).map(|i| self.hstrafe(i, d)).collect::<Vec<_>>(),
        };

        if moves.iter().all(|m| !m) {
            Err(Error::NoMoveOccurred)
        } else {
            self.gen_random_tile()
        }
    }

    fn vstrafe(&mut self, i: usize, d: Direction) -> bool {
        let mut col = self.tiles.column_mut(i);
        let mut moved = false;

        let (start, end, step): (isize, isize, isize) =
            match d {
                Direction::Up   => (0, 4, 1),
                Direction::Down => (3, -1, -1),
                _ => unreachable!()
            };

        let mut i: isize = start;

        'outer: while i != end {
            let mut j: isize = start;

            'inner: while j != i {
                let obstacle = col.rows((j+step) as usize, ((i-j).abs()) as usize - 1)
                    .iter().any(|&e| e != 0);

                let a = i as usize;
                let b = j as usize;

                if !obstacle && col[a] != 0 && (col[b] == 0 || col[b] == col[a]) {
                    col[b] += col[a];
                    col[a] = 0;
                    moved = true;
                    break 'inner;
                }

                j += step;
            }

            i += step;
        }

        moved
    }

    fn hstrafe(&mut self, i: usize, d: Direction) -> bool {
        let mut row = self.tiles.row_mut(i);
        let mut moved = false;

        let (start, end, step): (isize, isize, isize) =
            match d {
                Direction::Left  => (0, 4, 1),
                Direction::Right => (3, -1, -1),
                _ => unreachable!()
            };

        let mut i: isize = start;

        'outer: while i != end {
            let mut j: isize = start;

            'inner: while j != i {
                let obstacle = row.columns((j+step) as usize, ((i-j).abs()) as usize - 1)
                    .iter().any(|&e| e != 0);

                let a = i as usize;
                let b = j as usize;

                if !obstacle && row[a] != 0 && (row[b] == 0 || row[b] == row[a]) {
                    row[b] += row[a];
                    row[a] = 0;
                    moved = true;
                    break 'inner;
                }

                j += step;
            }

            i += step;
        }

        moved
    }

    fn get_empty_tile(&mut self) -> Result<&mut u32> {
        if self.is_over() {
            return Err(Error::NoMoreSpace);
        }

        loop {
            let i = rand::thread_rng().gen_range(0, 16);

            if self.tiles[(i / 4, i % 4)] == 0 {
                return Ok(&mut self.tiles[(i / 4, i % 4)]);
            }
        }
    }

    fn gen_random_tile(&mut self) -> Result<()> {
        let tile = self.get_empty_tile()?;
        *tile = rand::thread_rng().gen_range(1, 3) * 2;
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            for _ in 0..4 { write!(f, "+------")?; };                    writeln!(f, "+")?;
            for _ in 0..4 { write!(f, "|      ")?; };                    writeln!(f, "|")?;
            for j in 0..4 { write!(f, "| {:4} ", self.tiles[(i,j)])?; }; writeln!(f, "|")?;
            for _ in 0..4 { write!(f, "|      ")?; };                    writeln!(f, "|")?;
        }

        for _ in 0..4 { write!(f, "+------")?; }; writeln!(f, "+")
    }
}
