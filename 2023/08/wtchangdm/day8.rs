// revised per https://github.com/rust-tw/advent-of-code/pull/88
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("come on it's impossible"),
        }
    }
}

struct Map {
    cursor: usize,
    directions: Vec<Direction>,
    nodes: HashMap<String, [String; 2]>,
}

impl Map {
    fn new(input: &[&str]) -> Self {
        let mut nodes = HashMap::new();
        let mut iter = input.iter();
        let directions: Vec<Direction> = iter.next().unwrap().chars().map(Into::into).collect();

        iter.next();

        for line in iter {
            let (current, adjacents) = line.split_once(" = ").unwrap();
            let adjacents = adjacents.replace(['(', ')'], "");
            let (left, right) = adjacents.split_once(", ").unwrap();

            nodes
                .entry(current.into())
                .or_insert([left.to_string(), right.to_string()]);
        }

        Self {
            directions,
            nodes,
            cursor: 0,
        }
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn reset_cursor(&mut self) {
        self.cursor = 0
    }

    fn next_direction(&mut self) -> Direction {
        let direction = self.directions[self.cursor() % self.directions.len()];
        self.cursor += 1;
        direction
    }
}

pub fn solve_part1(input: &[&str]) -> u64 {
    let mut map = Map::new(input);
    let mut current = "AAA".to_string();

    while current != "ZZZ" {
        current = match map.next_direction() {
            Direction::Left => map.nodes.get(&current).unwrap()[0].clone(),
            Direction::Right => map.nodes.get(&current).unwrap()[1].clone(),
        };
    }

    map.cursor() as u64
}

pub fn solve_part2(input: &[&str]) -> u64 {
    let mut map = Map::new(input);
    let mut starting_nodes: Vec<String> = map
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();

    let mut steps = vec![0; starting_nodes.len()];

    for (i, node) in starting_nodes.iter_mut().enumerate() {
        while !node.ends_with('Z') {
            *node = match map.next_direction() {
                Direction::Left => map.nodes.get(node).unwrap()[0].clone(),
                Direction::Right => map.nodes.get(node).unwrap()[1].clone(),
            };
        }
        steps[i] = map.cursor() as u64;
        map.reset_cursor();
    }

    steps.iter().fold(1, lcm)
}

// Huge thanks to ChatGPT for the following helper functions
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: &u64) -> u64 {
    a / gcd(a, *b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part1_with_repeat_directions() {
        let input = [
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(solve_part1(&input), 6);
    }

    #[test]
    fn test_part2() {
        let input = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];

        assert_eq!(solve_part2(&input), 6);
    }
}
