pub fn day1_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .fold((None, None), |acc, x| match acc {
                    (None, _) => (Some(x), Some(x)),
                    _ => (acc.0, Some(x)),
                });

            nums.0.unwrap() * 10 + nums.1.unwrap()
        })
        .sum()
}

pub fn day1_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .enumerate()
                .filter_map(|(i, _)| match &line[i..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    s => s.chars().next().and_then(|c| c.to_digit(10)),
                })
                .fold((None, None), |acc, x| match acc {
                    (None, _) => (Some(x), Some(x)),
                    _ => (acc.0, Some(x)),
                });

            nums.0.unwrap() * 10 + nums.1.unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example1");

        assert_eq!(day1_part1(input), 142);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../example2");

        assert_eq!(day1_part2(input), 281);
    }
}
