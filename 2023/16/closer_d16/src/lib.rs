use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Visited {
    right: bool,
    down: bool,
    left: bool,
    up: bool,
}

impl Visited {
    fn any(&self) -> bool {
        self.right || self.down || self.left || self.up
    }
}

impl Default for Visited {
    fn default() -> Self {
        Visited {
            right: false,
            down: false,
            left: false,
            up: false,
        }
    }
}

impl Index<Dir> for Visited {
    type Output = bool;

    fn index(&self, index: Dir) -> &Self::Output {
        match index {
            Dir::Right => &self.right,
            Dir::Down => &self.down,
            Dir::Left => &self.left,
            Dir::Up => &self.up,
        }
    }
}

impl IndexMut<Dir> for Visited {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        match index {
            Dir::Right => &mut self.right,
            Dir::Down => &mut self.down,
            Dir::Left => &mut self.left,
            Dir::Up => &mut self.up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    kind: char,
    visited: Visited,
}

impl Grid {
    pub fn new(kind: char) -> Self {
        Grid {
            kind,
            visited: Visited::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    m: Vec<Vec<Grid>>,
    pub height: usize,
    pub width: usize,
}

impl Map {
    pub fn with_lines(lines: &Vec<&str>) -> Self {
        let m = lines
            .iter()
            .map(|line| line.chars().map(Grid::new).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = m.len();
        let width = m.get(0).map(|r| r.len()).unwrap_or(0);

        Map { m, height, width }
    }

    pub fn traverse(mut self, start: (usize, usize), dir: Dir) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((start, dir));

        while !queue.is_empty() {
            let ((y, x), d) = queue.pop_front().unwrap();
            self.m[y][x].visited[d] = true;

            let cands = match self.m[y][x].kind {
                '-' => match d {
                    Dir::Left | Dir::Right => vec![d],
                    Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                },
                '|' => match d {
                    Dir::Up | Dir::Down => vec![d],
                    Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
                },
                '/' => match d {
                    Dir::Right => vec![Dir::Up],
                    Dir::Down => vec![Dir::Left],
                    Dir::Left => vec![Dir::Down],
                    Dir::Up => vec![Dir::Right],
                },
                '\\' => match d {
                    Dir::Right => vec![Dir::Down],
                    Dir::Down => vec![Dir::Right],
                    Dir::Left => vec![Dir::Up],
                    Dir::Up => vec![Dir::Left],
                },
                _ => vec![d],
            };

            cands
                .into_iter()
                .filter_map(|d| self.try_forward((y, x), d))
                .filter(|&((y, x), d)| !self.m[y][x].visited[d])
                .for_each(|item| queue.push_back(item));
        }
        self
    }

    pub fn energized_count(&self) -> usize {
        self.m
            .iter()
            .map(|row| {
                row.iter()
                    .map(|g| if g.visited.any() { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }

    fn try_forward(&self, cur: (usize, usize), toward: Dir) -> Option<((usize, usize), Dir)> {
        type Signed = i64;
        let (y, x) = (cur.0 as Signed, cur.1 as Signed);
        let new_pos = match toward {
            Dir::Right => (y, x + 1),
            Dir::Down => (y + 1, x),
            Dir::Left => (y, x - 1),
            Dir::Up => (y - 1, x),
        };
        let h = self.height as Signed;
        let w = self.width as Signed;
        if (0..h).contains(&new_pos.0) && (0..w).contains(&new_pos.1) {
            Some(((new_pos.0 as usize, new_pos.1 as usize), toward))
        } else {
            None
        }
    }
}
