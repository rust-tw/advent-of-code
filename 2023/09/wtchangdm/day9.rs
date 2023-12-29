use std::collections::VecDeque;

type Series = Vec<VecDeque<i64>>;

fn get_series(history: &str) -> Series {
    let mut series: Series = vec![history
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()];

    while let Some(last) = series.last() {
        if last.iter().all(|&r| r == 0) {
            break;
        }

        let diff = last
            .iter()
            .zip(last.iter().skip(1))
            .map(|(&a, &b)| b - a)
            .collect();

        series.push(diff);
    }

    series
}

fn extrapolate(series: &mut Series) -> i64 {
    for i in (0..series.len() - 1).rev() {
        let val = series[i].back().unwrap() + series[i + 1].back().unwrap();
        series[i].push_back(val);
    }

    *series.first().unwrap().back().unwrap()
}

fn backtrack(series: &mut Series) -> i64 {
    for i in (0..series.len() - 1).rev() {
        let val = series[i].front().unwrap() - series[i + 1].front().unwrap();
        series[i].push_front(val);
    }

    *series.first().unwrap().front().unwrap()
}

pub fn solve_part1(input: &[&str]) -> i64 {
    input
        .iter()
        .map(|&line| get_series(line))
        .map(|mut s| extrapolate(&mut s))
        .sum()
}

pub fn solve_part2(input: &[&str]) -> i64 {
    input
        .iter()
        .map(|&line| get_series(line))
        .map(|mut s| backtrack(&mut s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

        assert_eq!(solve_part1(&input), 114);
    }

    #[test]
    fn test_part2() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

        assert_eq!(solve_part2(&input), 2);
    }
}
