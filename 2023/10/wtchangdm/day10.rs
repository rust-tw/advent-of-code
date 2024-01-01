use self::Direction::*;
use self::Tile::*;
use std::collections::HashSet;

type Point = (usize, usize);
type Offset = (isize, isize);

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_offset(&self) -> Offset {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    PipeNorthSouth,
    PipeWestEast,
    PipeNorthEast,
    PipeNorthWest,
    PipeSouthEast,
    PipeSouthWest,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '|' => PipeNorthSouth,
            '-' => PipeWestEast,
            'L' => PipeNorthEast,
            'J' => PipeNorthWest,
            'F' => PipeSouthEast,
            '7' => PipeSouthWest,
            '.' => Ground,
            'S' => Start,
            _ => panic!("error parsing tile from '{}' 😩", ch),
        }
    }
}

impl Tile {
    fn can_connect(&self, direction: Direction, other: Tile) -> bool {
        match (self, direction, other) {
            (_, _, Ground) => false,
            (_, _, Start) => true,

            (Start, Up, PipeNorthSouth | PipeSouthEast | PipeSouthWest) => true,
            (Start, Down, PipeNorthSouth | PipeNorthEast | PipeNorthWest) => true,
            (Start, Left, PipeWestEast | PipeNorthEast | PipeSouthEast) => true,
            (Start, Right, PipeWestEast | PipeNorthWest | PipeSouthWest) => true,

            (PipeNorthSouth, Up, PipeNorthSouth | PipeSouthEast | PipeSouthWest) => true,
            (PipeNorthSouth, Down, PipeNorthSouth | PipeNorthEast | PipeNorthWest) => true,

            (PipeWestEast, Left, PipeWestEast | PipeNorthEast | PipeSouthEast) => true,
            (PipeWestEast, Right, PipeWestEast | PipeNorthWest | PipeSouthWest) => true,

            (PipeNorthEast, Up, PipeNorthSouth | PipeSouthEast | PipeSouthWest) => true,
            (PipeNorthEast, Right, PipeWestEast | PipeNorthWest | PipeSouthWest) => true,

            (PipeNorthWest, Up, PipeNorthSouth | PipeSouthEast | PipeSouthWest) => true,
            (PipeNorthWest, Left, PipeWestEast | PipeNorthEast | PipeSouthEast) => true,

            (PipeSouthEast, Down, PipeNorthSouth | PipeNorthEast | PipeNorthWest) => true,
            (PipeSouthEast, Right, PipeWestEast | PipeNorthWest | PipeSouthWest) => true,

            (PipeSouthWest, Down, PipeNorthSouth | PipeNorthEast | PipeNorthWest) => true,
            (PipeSouthWest, Left, PipeWestEast | PipeNorthEast | PipeSouthEast) => true,

            _ => false,
        }
    }
}

struct Grid {
    start: Point,
    layout: Vec<Vec<Tile>>,
    traces: Vec<Point>,
}

impl Grid {
    fn from(input: &[&str]) -> Self {
        let mut start = (0, 0);
        let layout = input
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, char)| {
                        let tile = Tile::from(char);
                        if tile == Tile::Start {
                            start = (row, col);
                        }
                        tile
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<_>>();

        Self {
            layout,
            start,
            traces: vec![],
        }
    }

    fn traverse(&mut self, current: Point, prev: Direction, visited: &mut HashSet<Point>) -> bool {
        let (row, col) = current;
        let current_tile = self.layout[row][col];

        if visited.contains(&current) {
            return false;
        }

        visited.insert(current);
        self.traces.push(current);

        for direction in [Up, Down, Right, Left] {
            if direction == prev.opposite() {
                continue;
            }

            let (offset_row, offset_col) = direction.get_offset();

            let next_row = match row.checked_add_signed(offset_row) {
                Some(v) => v,
                None => continue,
            };

            let next_col = match col.checked_add_signed(offset_col) {
                Some(v) => v,
                None => continue,
            };

            if next_row >= self.layout.len() || next_col >= self.layout[0].len() {
                continue;
            }

            let next_tile = self.layout[next_row][next_col];
            let next = (next_row, next_col);

            if next == self.start {
                return true;
            }

            if !current_tile.can_connect(direction, next_tile) {
                continue;
            }

            if self.traverse(next, direction, visited) {
                return true;
            }
        }

        visited.remove(&current);
        self.traces.pop();

        false
    }
}

pub fn solve_part1(input: &[&str]) -> usize {
    let mut grid = Grid::from(input);
    grid.traverse(grid.start, Up, &mut HashSet::new());

    grid.traces.len() / 2
}

pub fn solve_part2(_input: &[&str]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        #[rustfmt::skip]
        let input = [
            ".....",
            ".S-7.",
            ".|.|.",
            ".L-J.",
            "....."
        ];

        assert_eq!(solve_part1(&input), 4);
    }

    #[test]
    fn test_part1_complex() {
        #[rustfmt::skip]
        let input = [
            "..F7.",
            ".FJ|.",
            "SJ.L7",
            "|F--J",
            "LJ..."
        ];

        assert_eq!(solve_part1(&input), 8);
    }

    #[test]
    fn test_part1_complex_2() {
        #[rustfmt::skip]
        let input = [
            "7-F7-",
            ".FJ|7",
            "SJLL7",
            "|F--J",
            "LJ.LJ",
        ];

        assert_eq!(solve_part1(&input), 8);
    }

    // #[test]
    fn test_part2() {
        #[rustfmt::skip]
        let input = [
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJIF7FJ-",
            "L---JF-JLJIIIIFJLJJ7",
            "|F|F-JF---7IIIL7L|7|",
            "|FFJF7L7F-JF7IIL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ];

        assert_eq!(solve_part2(&input), 10);
    }
}
