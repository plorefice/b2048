use std::fmt::{self, Display};
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
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut board = Board {
            tiles: vec![vec![0; 4]; 4],
            size
        };

        board.spawn_random().unwrap();

        board
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        !self.tiles.iter().flat_map(|r| r).any(|&t| t == 0)
    }

    pub fn swipe(&mut self, dir: Direction) -> Result<()> {
        let mut moved = false;

        for i in 0 .. self.size {
            let s = match dir {
                Direction::Up    => self.column_mut(i),
                Direction::Down  => self.column_mut(i).reverse(),
                Direction::Left  => self.row_mut(i),
                Direction::Right => self.row_mut(i).reverse(),
            };

            moved |= Board::squash(s)
        };

        if !moved {
            Err(Error::InvalidMove)
        } else {
            self.spawn_random()
                .and(Some(()))
                .ok_or(Error::BoardFull)
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

#[derive(Debug)]
enum HBorder {
    Top,
    Inner,
    Bottom,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0 .. self.size {
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

impl Board {
    fn size_hint(&self) -> (usize, usize) {
        (7 * self.size + 1, 4 * self.size + 1)
    }

    fn fmt_horiz_border(&self, level: HBorder) -> String {
        let mut s = String::new();

        let (lr, cr, rr) = match level {
            HBorder::Top    => ('┌', '┬', '┐'),
            HBorder::Inner  => ('├', '┼', '┤'),
            HBorder::Bottom => ('└', '┴', '┘'),
        };

        s.push_str(&format!("{}──────", lr));
        for _ in 1 .. self.size {
            s.push_str(&format!("{}──────", cr));
        };
        s.push_str(&format!("{}", rr));

        s
    }

    fn fmt_inner_row(&self, i: usize) -> String {
        let mut s = String::new();

        for _ in 0 .. self.size {
            s.push_str("│      ");
        };
        s.push_str("│\n");

        for j in 0 .. self.size {
            s.push_str(&format!("│ {:^4} ", self[(i,j)]));
        };
        s.push_str("│\n");

        for _ in 0 .. self.size {
            s.push_str("│      ");
        };
        s.push_str("│");

        s
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
