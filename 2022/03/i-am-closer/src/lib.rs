use std::collections::HashSet;

pub fn challenge_03(lines: &Vec<&str>) -> (u64, u64) {
    let part1 = lines.iter().map(|&s| handle_rucksack(s)).sum();
    let part2 = lines.chunks(3).map(|x| find_badge(&x.to_vec())).sum();

    (part1, part2)
}

fn handle_rucksack(rucksack: &str) -> u64 {
    let length = rucksack.len();
    let (c0, c1) = rucksack.split_at(length / 2);
    let set0 = c0.chars().collect::<HashSet<_>>();
    let set1 = c1.chars().collect::<HashSet<_>>();
    prioritize(set0.intersection(&set1).next())
}

fn find_badge(rucksacks: &Vec<&str>) -> u64 {
    let badge = rucksacks
        .iter()
        .map(|&s| s.chars().collect::<HashSet<_>>())
        // cloned() is used since we want HashMap<char> instead of HashMap<&char>
        .reduce(|accu, set| accu.intersection(&set).cloned().collect())
        .unwrap_or(HashSet::default());
    prioritize(badge.iter().next())
}

fn prioritize(item: Option<&char>) -> u64 {
    match item {
        Some(&c) if c.is_ascii_lowercase() => (c as u64) - ('a' as u64) + 1,
        Some(&c) if c.is_ascii_uppercase() => (c as u64) - ('A' as u64) + 27,
        _ => 0,
    }
}

#[test]
fn test_single_rucksack() {
    assert_eq!(handle_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(handle_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(handle_rucksack("PmmdzqPrVvPwwTWBwg"), 42);
    assert_eq!(handle_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
    assert_eq!(handle_rucksack("ttgJtRGJQctTZtZT"), 20);
    assert_eq!(handle_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
}

#[test]
fn test_find_badge() {
    let test_1 = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    ];
    let test_2 = vec![
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];
    assert_eq!(find_badge(&test_1), 18);
    assert_eq!(find_badge(&test_2), 52);
}
