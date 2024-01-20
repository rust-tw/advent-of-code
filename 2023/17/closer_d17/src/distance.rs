use super::Dir;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Dist {
    pub vertical: u64,
    pub horizontal: u64,
}

impl Default for Dist {
    fn default() -> Self {
        Dist {
            vertical: u64::MAX,
            horizontal: u64::MAX,
        }
    }
}

impl Index<Dir> for Dist {
    type Output = u64;

    fn index(&self, index: Dir) -> &Self::Output {
        match index {
            Dir::Right | Dir::Left => &self.horizontal,
            Dir::Down | Dir::Up => &self.vertical,
        }
    }
}

impl IndexMut<Dir> for Dist {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        match index {
            Dir::Right | Dir::Left => &mut self.horizontal,
            Dir::Down | Dir::Up => &mut self.vertical,
        }
    }
}
