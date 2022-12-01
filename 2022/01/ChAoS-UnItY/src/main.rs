use std::iter::Map;

fn main() {
    let input: &'static str = include_str!("../Day01.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn process_input(input: &'static str) -> Vec<i32> {
    let mut inputs = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|datas| {
            datas.split("\n")
                .map(|i| {
                    i.parse::<i32>().unwrap()
                })
                .sum()
        })
        .collect::<Vec<i32>>();
    
    inputs.sort();
    
    return inputs;
}

fn part1(input: &'static str) -> i32 {
    *process_input(input).iter().max().unwrap()
}

fn part2(input: &'static str) -> i32 {
    process_input(input).iter().rev().take(3).sum()
}
