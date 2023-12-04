use day03::{day3_part1, day3_part2};



fn main(){
    let input = include_str!("../input");

    println!("Part1 answer: {}", day3_part1(input));
    println!("Part2 answer: {}", day3_part2(input));
}