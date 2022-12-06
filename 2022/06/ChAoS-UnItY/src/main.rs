use std::collections::HashSet;

fn main() {
    let data = include_str!("../Day06.txt");

    println!("{}", part1(data));
    println!("{}", part2(data));
}

fn part1(str: &'static str) -> usize {
    str.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, b)| is_duplicate(*b))
        .map(|(i, _)| i + 4)
        .unwrap()
}

fn part2(str: &'static str) -> usize {
    str.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, b)| is_duplicate(*b))
        .map(|(i, _)| i + 14)
        .unwrap()
}

fn is_duplicate(bytes: &[u8]) -> bool {
    bytes.iter().collect::<HashSet<_>>().len() != bytes.len()
}
