use std::collections::HashMap;

fn main() {
    let content = include_str!("../Day01.txt");
    part1(content);
    part2(content);
}

fn part1(content: &str) {
    let mut llist = vec![];
    let mut rlist = vec![];

    content.split("\n")
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(l, r)| {
            llist.push(l.parse::<u32>().unwrap());
            rlist.push(r.parse::<u32>().unwrap());
        });

    llist.sort();
    rlist.sort();

    let result = llist.into_iter().zip(rlist.into_iter())
        .fold(0u32, |acc, (l, r)| {
            acc + l.abs_diff(r)
        });

    println!("Part 1: {result}");
}

fn part2(content: &str) {
    let mut llist = vec![];
    let mut rmap = HashMap::new();

    content.split("\n")
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(l, r)| {
            llist.push(l.parse::<u32>().unwrap());
            rmap.entry(r.parse::<u32>().unwrap()).and_modify(|c: &mut u32| *c += 1).or_insert(1);
        });

    let result = llist.into_iter()
        .fold(0u32, |acc, e| {
            acc + rmap.get(&e).copied().unwrap_or_default() * e
        });

    println!("Part 2: {result}");
}
