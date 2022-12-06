use std::collections::HashSet;

fn main() {
    let data = include_str!("../Day06.txt");

    println!("{}", part1(data));
    println!("{}", part2(data));
}

fn part1(str: &'static str) -> usize {
    for count in 4 .. str.len() {
        if !is_duplicate(&str[count - 4 .. count]) {
            return count;
        }
    }

    return 0
}

fn part2(str: &'static str) -> usize {
    for count in 14 .. str.len() {
        if !is_duplicate(&str[count - 14 .. count]) {
            return count;
        }
    }

    return 0
}

fn is_duplicate(str: &str) -> bool {
    str.as_bytes().iter().collect::<HashSet<_>>().len() != str.as_bytes().len()
}
