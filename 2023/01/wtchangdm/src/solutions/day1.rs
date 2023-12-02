fn transform_words_to_digits(line: &str) -> String {
    line.chars()
        .enumerate()
        .map(|(i, _)| {
            let letter = line.chars().nth(i).unwrap();

            match &line[i..] {
                _ if letter.is_numeric() => letter,
                x if x.starts_with("one") => '1',
                x if x.starts_with("two") => '2',
                x if x.starts_with("three") => '3',
                x if x.starts_with("four") => '4',
                x if x.starts_with("five") => '5',
                x if x.starts_with("six") => '6',
                x if x.starts_with("seven") => '7',
                x if x.starts_with("eight") => '8',
                x if x.starts_with("nine") => '9',
                _ => '\0',
            }
        })
        .collect()
}

fn get_number(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    format!("{}{}", first, last).parse().unwrap()
}

pub fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|s| get_number(s)).sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| transform_words_to_digits(s))
        .map(|s| get_number(&s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 281);
    }
}
