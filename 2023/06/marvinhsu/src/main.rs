use day06::{day6_part1_v2, day6_part2_v2};

fn main() {
    let input = include_str!("../input");
    println!("Part1: {}", day6_part1_v2(input).unwrap());
    println!("Part2: {}", day6_part2_v2(input).unwrap());
}
