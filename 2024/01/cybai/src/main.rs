use std::collections::HashMap;

fn main() {
    let input = include_str!("../day01.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();

            (a, b)
        })
        .unzip();

    lefts.sort();
    rights.sort();

    lefts
        .into_iter()
        .zip(rights)
        .fold(0, |acc, (a, b)| acc + (b - a).abs())
}

fn part2(input: &str) -> i32 {
    let (lefts, rights): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();

            (a, b)
        })
        .unzip();

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for n in &lefts {
        if !counts.contains_key(n) {
            counts.insert(*n, 0);
        }
    }

    for n in rights {
        if let Some(count) = counts.get_mut(&n) {
            *count += 1;
        }
    }

    lefts.into_iter().map(|n| n * counts[&n]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
