use aoc_2023::day01;

fn main() {
    let my_str = include_str!("day01-input");
    println!("{}", day01::solve1(my_str));
    println!("{}", day01::solve2(my_str));
}
