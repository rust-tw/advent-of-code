type Bag = Vec<Subset>;
type Subset = Vec<(u32, Color)>;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[derive(Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Corrupted input data: Unknown color `{value}`"),
        }
    }
}

fn main() {
    let content = include_str!("../Day02.txt");
    let sets = resolve_input(content);
    part1(&sets);
    part2(&sets);
}

fn resolve_input(content: &str) -> Vec<Bag> {
    content
        .split("\n")
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|per_config| per_config.split_once(" ").unwrap())
                        .map(|(count, color)| {
                            (count.parse::<u32>().unwrap(), Into::<Color>::into(color))
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(input: &Vec<Bag>) {
    let result = input
        .iter()
        .enumerate()
        .filter(|(_, bag)| {
            let mut red_counter = 0;
            let mut green_counter = 0;
            let mut blue_counter = 0;

            for subset in *bag {
                for &(count, color) in subset {
                    match color {
                        Color::Red => red_counter = red_counter.max(count),
                        Color::Green => green_counter = green_counter.max(count),
                        Color::Blue => blue_counter = blue_counter.max(count),
                    }
                }
            }

            red_counter <= RED_LIMIT && green_counter <= GREEN_LIMIT && blue_counter <= BLUE_LIMIT
        })
        .map(|(index, _)| index as u32 + 1)
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part2(input: &Vec<Bag>) {
    let result = input
        .iter()
        .map(|bag| {
            let mut red_counter = 0;
            let mut green_counter = 0;
            let mut blue_counter = 0;

            for subset in bag {
                for &(count, color) in subset {
                    match color {
                        Color::Red => red_counter = red_counter.max(count),
                        Color::Green => green_counter = green_counter.max(count),
                        Color::Blue => blue_counter = blue_counter.max(count),
                    }
                }
            }

            red_counter * green_counter * blue_counter
        })
        .sum::<u32>();

    println!("Part 1: {result}");
}
