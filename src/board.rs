use std::ops::{Index, IndexMut, Range};
use std::iter::FromIterator;
use std::result;

use rand::{self, Rng};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BoardFull,
    InvalidMove,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<u32>>,
    score: u32,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut board = Board {
            tiles: vec![vec![0; 4]; 4],
            score: 0,
            size
        };

        board.spawn_random().unwrap();

        board
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        !self.get_tiles().any(|&t| t == 0)
    }

    pub fn get_tiles(&self) -> impl Iterator<Item = &u32> {
        self.tiles.iter().flat_map(|r| r)
    }

    pub fn swipe(&mut self, dir: Direction) -> Result<u32> {
        let scores = (0 .. self.size).map(|i| {
            let s = match dir {
                Direction::Up    => self.column_mut(i),
                Direction::Down  => self.column_mut(i).reverse(),
                Direction::Left  => self.row_mut(i),
                Direction::Right => self.row_mut(i).reverse(),
            };

            Board::squash(s)
        }).collect::<Vec<_>>();

        let moved = scores.iter().any(|s| s.is_some());
        let score = scores.iter().map(|s| s.unwrap_or(0)).sum();

        if !moved && !self.is_full() {
            Err(Error::InvalidMove)
        } else {
            self.spawn_random()
                .and(Some(score))
                .ok_or(Error::BoardFull)
        }
    }

    fn squash(mut s: SliceMut) -> Option<u32> {
        let mut moved = false;
        let mut score = 0;

        'outer: for i in 0 .. s.len() {
            if s[i] == 0 {
                continue 'outer;
            }

            'inner: for j in 0 .. i {
                let obstacle_present = s[j+1 .. i].iter().any(|e| **e != 0);

                if !obstacle_present && (s[j] == 0 || s[i] == s[j]) {
                    score += s[j] * 2;
                    moved = true;
                    s[j] += s[i];
                    s[i] = 0;
                    break 'inner;
                }
            }
        }

        if moved {
            Some(score)
        } else {
            None
        }
    }

    fn spawn_random(&mut self) -> Option<(usize, usize)> {
        let idx = self.get_empty_tile()?;
        self[idx] = rand::thread_rng().gen_range(1, 3) * 2;
        Some(idx)
    }

    fn get_empty_tile(&mut self) -> Option<(usize, usize)> {
        let sz = self.size;

        if self.is_full() {
            return None;
        }

        loop {
            let i = rand::thread_rng().gen_range(0, sz * sz);

            if self[(i / sz, i % sz)] == 0 {
                return Some((i / sz, i % sz));
            }
        }
    }

    fn row_mut(&mut self, i: usize) -> SliceMut {
        self.tiles[i].iter_mut().collect::<SliceMut>()
    }

    fn column_mut(&mut self, i: usize) -> SliceMut {
        self.tiles.iter_mut().map(|r| &mut r[i]).collect::<SliceMut>()
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u32;

    fn index(&self, (r,c): (usize, usize)) -> &u32 {
        &self.tiles[r][c]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (r,c): (usize, usize)) -> &mut u32 {
        &mut self.tiles[r][c]
    }
}

#[derive(Debug)]
pub struct SliceMut<'a> {
    data: Vec<&'a mut u32>,
}

impl<'a> SliceMut<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn reverse(mut self) -> SliceMut<'a> {
        self.data.reverse();
        self
    }
}

impl<'a> FromIterator<&'a mut u32> for SliceMut<'a> {
    fn from_iter<T>(iter: T) -> SliceMut<'a>
        where T: IntoIterator<Item = &'a mut u32>
    {
        let mut data = Vec::new();

        for item in iter {
            data.push(item);
        };

        SliceMut { data }
    }
}

impl<'a> Index<usize> for SliceMut<'a> {
    type Output = u32;

    fn index(&self, i: usize) -> &u32 {
        self.data[i]
    }
}

impl<'a> Index<Range<usize>> for SliceMut<'a> {
    type Output = [&'a mut u32];

    fn index(&self, r: Range<usize>) -> &[&'a mut u32] {
        &self.data[r]
    }
}

impl<'a> IndexMut<usize> for SliceMut<'a> {
    fn index_mut(&mut self, i: usize) -> &mut u32 {
        &mut self.data[i]
    }
}

impl<'a> IndexMut<Range<usize>> for SliceMut<'a> {
    fn index_mut(&mut self, r: Range<usize>) -> &mut [&'a mut u32] {
        &mut self.data[r]
    }
}
