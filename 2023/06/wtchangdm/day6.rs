fn parse_one(s: &str) -> u64 {
    s.split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn parse(s: &str) -> Vec<u64> {
    s.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn solve(durations: Vec<u64>, records: Vec<u64>) -> u64 {
    durations
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let (duration, record) = (durations[i], records[i]);

            (1..duration)
                .filter(|hold| {
                    let distance = hold * (duration - hold);
                    distance > record
                })
                .count()
        })
        .map(|x| x as u64)
        .product()
}

pub fn solve_part1(input: &[String]) -> u64 {
    let (durations, records) = (parse(&input[0]), parse(&input[1]));

    solve(durations, records)
}

pub fn solve_part2(input: &[String]) -> u64 {
    let (duration, record) = (parse_one(&input[0]), parse_one(&input[1]));

    solve(vec![duration], vec![record])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = ["Time:      7  15   30", "Distance:  9  40  200"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_part1(&input), 288);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = ["Time:      7  15   30", "Distance:  9  40  200"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_part2(&input), 71503);
    }
}
