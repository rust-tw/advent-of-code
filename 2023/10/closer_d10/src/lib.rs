pub fn build_grids(input: &str) -> (Vec<Vec<Grid>>, usize, usize) {
    let mut grids = input
        .lines()
        .map(|line| line.chars().map(|c| Grid::from_char(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_y, start_x) = grids
        .iter()
        .enumerate()
        .find_map(|(idx, line)| {
            line.iter()
                .enumerate()
                .find(|(_, grid)| grid.is_start)
                .map(|(x, _)| (idx, x))
        })
        .unwrap();

    // Determines the pipe in the Start grid by checking its neighbors
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .for_each(|dir| {
        if let Some((y, x)) = dir.try_proceed(start_y, start_x, (grids.len(), grids[0].len())) {
            if grids[y][x].neighbors.contains(&dir.turn_180()) {
                grids[start_y][start_x].neighbors.push(dir);
            }
        }
    });

    (grids, start_y, start_x)
}

pub fn format_grids(grids: &Vec<Vec<Grid>>) -> String {
    grids
        .iter()
        .map(|grid_line| {
            grid_line
                .iter()
                .map(|grid| grid.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn try_proceed(&self, y: usize, x: usize, max: (usize, usize)) -> Option<(usize, usize)> {
        let y = y as i64;
        let x = x as i64;
        let (cand_y, cand_x) = match self {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        };

        if (0..(max.0 as i64)).contains(&cand_y) && (0..(max.1 as i64)).contains(&cand_x) {
            Some((cand_y as usize, cand_x as usize))
        } else {
            None
        }
    }

    pub fn turn_180(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Grid {
    pub neighbors: Vec<Direction>,
    pub distance: i32,
    pub visited: bool,
    pub is_start: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            neighbors: vec![],
            distance: 0,
            visited: false,
            is_start: false,
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let c = if self.is_start {
            'S'
        } else if !self.visited {
            '.'
        } else {
            if self.neighbors.contains(&Direction::Up) {
                if self.neighbors.contains(&Direction::Down) {
                    '│'
                } else if self.neighbors.contains(&Direction::Left) {
                    '┘'
                } else {
                    '└'
                }
            } else if self.neighbors.contains(&Direction::Down) {
                if self.neighbors.contains(&Direction::Left) {
                    '┐'
                } else {
                    '┌'
                }
            } else if self.neighbors.contains(&Direction::Left) {
                '─'
            } else {
                '.'
            }
            // self.distance.to_string().chars().last().unwrap()
        };

        format!("{c}")
    }
}

impl Grid {
    pub fn from_char(c: char) -> Self {
        match c {
            '|' => Grid {
                neighbors: vec![Direction::Up, Direction::Down],
                ..Self::default()
            },
            '-' => Grid {
                neighbors: vec![Direction::Left, Direction::Right],
                ..Self::default()
            },
            'L' => Grid {
                neighbors: vec![Direction::Up, Direction::Right],
                ..Self::default()
            },
            'J' => Grid {
                neighbors: vec![Direction::Up, Direction::Left],
                ..Self::default()
            },
            '7' => Grid {
                neighbors: vec![Direction::Down, Direction::Left],
                ..Self::default()
            },
            'F' => Grid {
                neighbors: vec![Direction::Down, Direction::Right],
                ..Self::default()
            },
            'S' => Grid {
                is_start: true,
                ..Self::default()
            },
            _ => Self::default(),
        }
    }
}
