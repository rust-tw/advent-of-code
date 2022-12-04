use std::collections::HashSet;

fn main() {
    let data = include_str!("../Day03.txt");

    println!("{}", part1(data));
    println!("{}", part2(data));
}

fn part1(data: &'static str) -> i32 {
    data.replace("\r\n", "\n")
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(l, r)| {
            l.chars()
                .into_iter()
                .collect::<HashSet<char>>()
                .intersection(&r.chars().into_iter().collect::<HashSet<char>>())
                .next()
                .map(|c| priority(c))
                .unwrap()
        })
        .sum()
}

fn part2(data: &'static str) -> i32 {
    data.replace("\r\n", "\n")
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|s| s.chars().into_iter().collect::<HashSet<char>>())
                .reduce(|acc, item| {
                    acc.intersection(&item)
                        .map(|c| c.clone())
                        .collect::<HashSet<char>>()
                })
                .and_then(|hs| {
                    hs.iter()
                        .next()
                        .map(|c| priority(c))
                })
                .unwrap()
        })
        .sum()
}

fn priority(c: &char) -> i32 {
    if c.is_lowercase() {
        *c as i32 - 97 + 1
    } else {
        *c as i32 - 65 + 27
    }
}
