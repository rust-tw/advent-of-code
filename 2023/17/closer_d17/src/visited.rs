use super::Dir;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Visited {
    vertical: bool,
    horizontal: bool,
}

impl Default for Visited {
    fn default() -> Self {
        Visited {
            vertical: false,
            horizontal: false,
        }
    }
}

impl Index<Dir> for Visited {
    type Output = bool;

    fn index(&self, index: Dir) -> &Self::Output {
        match index {
            Dir::Right | Dir::Left => &self.horizontal,
            Dir::Down | Dir::Up => &self.vertical,
        }
    }
}

impl IndexMut<Dir> for Visited {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        match index {
            Dir::Right | Dir::Left => &mut self.horizontal,
            Dir::Down | Dir::Up => &mut self.vertical,
        }
    }
}
