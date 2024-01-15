pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    grids: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Platform {
    pub fn parse(lines: &Vec<&str>) -> Self {
        let grids = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grids[0].len();
        let height = grids.len();

        Platform {
            grids,
            width,
            height,
        }
    }

    pub fn tilt(mut self, dir: Direction) -> Self {
        match dir {
            Direction::North => {
                for x in 0..self.width {
                    let mut stop = 0;
                    for y in 0..self.height {
                        match self.grids[y][x] {
                            'O' => {
                                let tmp = self.grids[stop][x];
                                self.grids[stop][x] = self.grids[y][x];
                                self.grids[y][x] = tmp;
                                stop += 1;
                            }
                            '#' => {
                                stop = y + 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Direction::West => {
                for y in 0..self.height {
                    let mut stop = 0;
                    for x in 0..self.width {
                        match self.grids[y][x] {
                            'O' => {
                                let tmp = self.grids[y][stop];
                                self.grids[y][stop] = self.grids[y][x];
                                self.grids[y][x] = tmp;
                                stop += 1;
                            }
                            '#' => {
                                stop = x + 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Direction::South => {
                for x in 0..self.width {
                    let mut stop = self.height - 1;
                    for y in (0..self.height).rev() {
                        match self.grids[y][x] {
                            'O' => {
                                let tmp = self.grids[stop][x];
                                self.grids[stop][x] = self.grids[y][x];
                                self.grids[y][x] = tmp;
                                stop = stop.saturating_sub(1);
                            }
                            '#' => {
                                stop = y.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }
            }
            Direction::East => {
                for y in 0..self.height {
                    let mut stop = self.width - 1;
                    for x in (0..self.width).rev() {
                        match self.grids[y][x] {
                            'O' => {
                                let tmp = self.grids[y][stop];
                                self.grids[y][stop] = self.grids[y][x];
                                self.grids[y][x] = tmp;
                                stop = stop.saturating_sub(1);
                            }
                            '#' => {
                                stop = x.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self
    }

    pub fn diff_rocks(&self, other: &Self) -> Vec<(usize, usize)> {
        if self.height == other.height && self.width == other.width {
            self.grids
                .iter()
                .zip(other.grids.iter())
                .enumerate()
                .map(|(y, (line_a, line_b))| {
                    line_a
                        .iter()
                        .zip(line_b.iter())
                        .enumerate()
                        .filter(|(_, (ca, cb))| **ca != **cb)
                        .map(|(x, _)| (y, x))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    pub fn eval(&self) -> u64 {
        self.grids
            .iter()
            .fold((0, self.height as u64), |(val, factor), row| {
                let row_val = row.iter().filter(|c| **c == 'O').count() as u64 * factor;
                (val + row_val, factor - 1)
            })
            .0
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn test_data() -> Vec<&'static str> {
        vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
    }

    #[test]
    fn test_tilt() {
        let platform = Platform::parse(&test_data())
            .tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East);

        let expected = Platform::parse(&vec![
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ]);

        assert_eq!(platform, expected);
    }

    #[test]
    fn test_eval() {
        let platform = Platform::parse(&test_data()).tilt(Direction::North);

        assert_eq!(platform.eval(), 136);
    }
}
