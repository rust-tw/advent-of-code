use day07::{part1, part2};

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input).unwrap());
    println!("Part 2: {}", part2(input).unwrap());
}
