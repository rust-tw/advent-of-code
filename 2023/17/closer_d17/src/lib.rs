mod distance;
mod visited;

use std::collections::BinaryHeap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::cmp::Reverse;
use distance::Dist;
use visited::Visited;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn cw(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn ccw(self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
        }
    }
}

pub struct Walker {
    grids: Vec<Vec<u64>>,
}

impl Walker {
    pub fn walk(&self, min_steps: usize, max_steps: usize) -> Vec<Vec<Dist>> {
        let height = self.grids.len();
        let width = self.grids[0].len();

        let mut dists = vec![vec![Dist::default(); width]; height];
        dists[0][0][Dir::Right] = 0;
        dists[0][0][Dir::Down] = 0;
        let mut visited = vec![vec![Visited::default(); width]; height];

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 0, 0, Dir::Right));
        queue.push((Reverse(0), 0, 0, Dir::Down));

        while let Some((Reverse(dist), y, x, dir)) = queue.pop() {
            if visited[y][x][dir] {
                continue;
            }
            visited[y][x][dir] = true;

            for ndir in [dir.cw(), dir.ccw()] {
                let mut pos = (y, x);
                let mut new_dist = dist;
                for steps in 0..max_steps {
                    if let Some(npos) = self.forward(ndir, pos.0, pos.1) {
                        new_dist += self.grids[npos.0][npos.1];
                        if steps >= min_steps - 1 {
                            if dists[npos.0][npos.1][ndir] > new_dist {
                                dists[npos.0][npos.1][ndir] = new_dist;
                                queue.push((Reverse(new_dist), npos.0, npos.1, ndir));
                            }
                        }
                        pos = npos;
                    } else {
                        break;
                    }
                }
            }
        }
        dists
    }

    fn forward(&self, dir: Dir, y: usize, x: usize) -> Option<(usize, usize)> {
        match dir {
            Dir::Right if x < self.grids[0].len() - 1 => Some((y, x + 1)),
            Dir::Down if y < self.grids.len() - 1 => Some((y + 1, x)),
            Dir::Left if x > 0 => Some((y, x - 1)),
            Dir::Up if y > 0 => Some((y - 1, x)),
            _ => None
        }
    }
}

impl FromStr for Walker {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grids: Vec<Vec<u64>> = s
            .lines()
            .map(|line| line.chars().map(|c| (c as u64) - ('0' as u64)).collect())
            .collect();

        if grids.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input."));
        }

        Ok(Self { grids })
    }
}
