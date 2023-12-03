use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // move right
    (0, -1),  // move left
    (1, 0),   // move down
    (-1, 0),  // move up
    (1, 1),   // move bottom-right
    (1, -1),  // move bottom-left
    (-1, 1),  // move up-right
    (-1, -1), // move up-left
];

type Point = (usize, usize);
type Part = u32;
type Symbol = char;

#[derive(Debug)]
struct Schematic {
    symbol_map: HashMap<char, HashMap<Point, Vec<Part>>>,
    parts_nearby_symbols: Vec<Part>,
}

fn get_nearby_symbols(m: &[Vec<char>], p: &Point) -> Vec<(Symbol, Point)> {
    let mut symbols = vec![];

    for (y, x) in DIRECTIONS {
        let next_y = match y.checked_add(p.1 as i32) {
            Some(v) => v as usize,
            None => continue,
        };

        let next_x = match x.checked_add(p.0 as i32) {
            Some(v) => v as usize,
            None => continue,
        };

        let character = m
            .get(next_y)
            .and_then(|_| m.get(next_y).unwrap().get(next_x));

        if let Some(c) = character {
            if !c.is_numeric() && *c != '.' {
                symbols.push((*c, (next_y, next_x)));
            }
        }
    }

    symbols
}

impl Schematic {
    fn from(input: &[String]) -> Self {
        let mut schematic = Self {
            symbol_map: HashMap::new(),
            parts_nearby_symbols: vec![],
        };

        let matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

        for (row, line) in matrix.iter().enumerate() {
            let mut s = String::new();
            let mut adjacent_symbols: HashSet<(char, (usize, usize))> = HashSet::new();
            let mut has_nearby_symbols = false;

            for (col, &char) in line.iter().enumerate() {
                match char {
                    c if c.is_numeric() => {
                        let point: Point = (col, row);
                        let nearby_symbols = get_nearby_symbols(&matrix, &point);

                        if !nearby_symbols.is_empty() {
                            has_nearby_symbols = true;
                        };

                        for (symbol, symbol_point) in nearby_symbols {
                            adjacent_symbols.insert((symbol, symbol_point));
                        }
                        s.push(c);
                    }
                    c if !c.is_numeric() && !s.is_empty() => {
                        let part: Part = s.parse().unwrap();

                        if has_nearby_symbols {
                            schematic.parts_nearby_symbols.push(part);
                        }

                        adjacent_symbols.iter().for_each(|(symbol, symbol_point)| {
                            schematic
                                .symbol_map
                                .entry(*symbol)
                                .or_default()
                                .entry(*symbol_point)
                                .or_default()
                                .push(part);
                        });

                        s = String::new();
                        adjacent_symbols.clear();
                        has_nearby_symbols = false;
                    }
                    _ => (),
                }
            }

            if !s.is_empty() {
                let part: Part = s.parse().unwrap();

                if has_nearby_symbols {
                    schematic.parts_nearby_symbols.push(part);
                }

                adjacent_symbols.iter().for_each(|(symbol, symbol_point)| {
                    schematic
                        .symbol_map
                        .entry(*symbol)
                        .or_default()
                        .entry(*symbol_point)
                        .or_default()
                        .push(part);
                });

                adjacent_symbols.clear();
            }
        }

        schematic
    }
}

pub fn solve_part1(input: &[String]) -> u32 {
    Schematic::from(input).parts_nearby_symbols.iter().sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    Schematic::from(input)
        .symbol_map
        .get_key_value(&'*')
        .unwrap()
        .1
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(dbg!(solve_part1(&input)), 4361);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(dbg!(solve_part2(&input)), 467835);
    }
}
