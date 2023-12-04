use day01::{day1_part1, day1_part2};

fn main(){
    let input = include_str!("../input");

    println!("Part1 answer: {}", day1_part1(input));
    println!("Part2 answer: {}", day1_part2(input));
}