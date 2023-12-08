use anyhow::{anyhow, Result};
use itertools::Itertools;
#[allow(unused_imports)]
use tap::Tap;

#[allow(dead_code)]
fn day6_part1_v1(input: &str) -> Result<usize> {
    let rounds = parse_round(input)?;

    let result = rounds
        .iter()
        .map(|round| {
            round
                .get_win_times()
                // .tap(|v| println!("{:?}", v))
                .len()
            // .tap(|l| println!("{}: {}", round.time, l))
        })
        .product();

    Ok(result)
}

pub fn day6_part1_v2(input: &str) -> Result<usize> {
    let rounds = parse_round(input)?;

    let result = rounds
        .iter()
        .map(|round| {
            // let function = | x | x * (round.time - x);
            // a = 1; b = -round.time; c = round.distance;

            round
                .time
                .pow(2)
                .checked_add_signed(-4 * round.distance as isize)
                .ok_or(anyhow!("Overflow"))
                .map(|discriminant| {
                    let b = round.time as f64;
                    let factor = (discriminant as f64).sqrt();
                    let start = (b - factor) / 2.0;
                    let end = (b + factor) / 2.0;

                    let start = (if start.ceil() == start {
                        start + 1.0
                    } else {
                        start.ceil()
                    }) as usize;
                    let end = end.ceil() as usize;

                    (start..end).len()
                })
        })
        .process_results(|iter| iter.product::<usize>())?;

    Ok(result)
}

#[allow(dead_code)]
fn day6_part2_v1(input: &str) -> Result<usize> {
    let rounds = parse_round(&input.replace(' ', ""))?;

    let result = rounds
        .iter()
        .map(|round| {
            round
                .get_win_times()
                // .tap(|v| println!("{:?}", v))
                .len()
            // .tap(|l| println!("{}: {}", round.time, l))
        })
        .sum();

    Ok(result)
}

pub fn day6_part2_v2(input: &str) -> Result<usize> {
    let rounds = parse_round(&input.replace(' ', ""))?;

    let result = rounds
        .iter()
        .map(|round| {
            // let function = | x | x * (round.time - x);
            // a = 1; b = -round.time; c = round.distance;

            round
                .time
                .pow(2)
                .checked_add_signed(-4 * round.distance as isize)
                .ok_or(anyhow!("Overflow"))
                .map(|discriminant| {
                    let b = round.time as f64;
                    let factor = (discriminant as f64).sqrt();
                    let start = (b - factor) / 2.0;
                    let end = (b + factor) / 2.0;

                    let start = (if start.ceil() == start {
                        start + 1.0
                    } else {
                        start.ceil()
                    }) as usize;
                    let end = end.ceil() as usize;

                    (start..end).len()
                })
        })
        .process_results(|iter| iter.sum())?;

    Ok(result)
}

#[derive(Debug, PartialEq)]
struct Round {
    time: usize,
    distance: usize,
}

impl Round {
    fn get_win_times(&self) -> Vec<usize> {
        (0..self.time)
            .map(|t| {
                let speed = t;
                let time = self.time - t;
                (t, speed * time)
            })
            .filter_map(|(t, d)| if d > self.distance { Some(t) } else { None })
            .collect::<_>()
    }
}

fn parse_round(input: &str) -> Result<Vec<Round>> {
    let (time, distance) = input.split_once('\n').ok_or(anyhow!("No newline"))?;
    time.split_once(':')
        .ok_or(anyhow!("No ':' in time"))?
        .1
        .split_ascii_whitespace()
        .zip(
            distance
                .split_once(':')
                .ok_or(anyhow!("No ':' in distance"))?
                .1
                .split_ascii_whitespace(),
        )
        .map(|(t, d)| {
            let time = t.trim().parse::<usize>()?;
            let distance = d.trim().parse::<usize>()?;
            Ok(Round { time, distance })
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = include_str!("../example");
        assert_eq!(day6_part1_v1(input).unwrap(), 288);
        assert_eq!(day6_part1_v2(input).unwrap(), 288);
    }

    #[test]
    fn test_day6_part2() {
        let input = include_str!("../example");
        assert_eq!(day6_part2_v1(input).unwrap(), 71503);
        assert_eq!(day6_part2_v2(input).unwrap(), 71503);
    }

    #[test]
    fn test_parse_round() {
        let input = include_str!("../example");
        let expected = vec![
            Round {
                time: 7,
                distance: 9,
            },
            Round {
                time: 15,
                distance: 40,
            },
            Round {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(parse_round(input).unwrap(), expected);
    }

    #[test]
    fn test_get_win_times() {
        let round = Round {
            time: 7,
            distance: 9,
        };
        let expected = vec![2, 3, 4, 5];
        assert_eq!(round.get_win_times(), expected);
    }
}
