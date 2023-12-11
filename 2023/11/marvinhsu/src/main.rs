use day11::{part1, part2};

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(input).unwrap());
    println!("part2: {}", part2(input).unwrap());
}
