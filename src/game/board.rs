use std::ops::{Index, IndexMut, Range};
use std::iter::FromIterator;

use rand::{self, Rng};

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<u32>>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            tiles: vec![vec![0; 4]; 4],
            size
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        !self.tiles.iter().flat_map(|r| r).any(|&t| t == 0)
    }

    pub fn spawn_random(&mut self) -> Option<(usize, usize)> {
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

    pub fn row_mut(&mut self, i: usize) -> SliceMut {
        self.tiles[i].iter_mut().collect::<SliceMut>()
    }

    pub fn column_mut(&mut self, i: usize) -> SliceMut {
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
