use day02::{day2_part1, day2_part2};

fn main(){
    let input = include_str!("../input");

    println!("Part1 answer: {}", day2_part1(input));
    println!("Part2 answer: {}", day2_part2(input));
}