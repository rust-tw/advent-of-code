use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<isize> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().context("Failed to parse input"))
                .collect::<Result<Vec<_>>>()
        })
        .process_results(|iter| iter.map(|layer| get_next_number(layer.as_slice())).sum())
}

pub fn part2(input: &str) -> Result<isize> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().context("Failed to parse input"))
                .collect::<Result<Vec<_>>>()
        })
        .process_results(|iter| {
            iter.map(|layer| get_previous_number(layer.as_slice()))
                .sum()
        })
}

fn get_sub_layer(layer: &[isize]) -> Option<Vec<isize>> {
    match layer {
        [.., 0] => None,
        _ => Some(
            layer
                .iter()
                .zip(layer.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect_vec(),
        ),
    }
}

fn get_next_number(layer: &[isize]) -> isize {
    let last = layer[layer.len() - 1];
    match get_sub_layer(layer) {
        Some(sub_layer) => last + get_next_number(sub_layer.as_slice()),
        None => last,
    }
}

fn get_previous_number(layer: &[isize]) -> isize {
    let first = layer[0];
    match get_sub_layer(layer) {
        Some(sub_layer) => first - get_previous_number(sub_layer.as_slice()),
        None => first,
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example");
        assert_eq!(part1(input).unwrap(), 114);
    }

    #[test]
    fn test_get_sub_layer() {
        assert_eq!(
            get_sub_layer(&vec![1, 3, 6, 10, 15, 21]).unwrap(),
            [2, 3, 4, 5, 6]
        );
        assert_eq!(get_sub_layer(&vec![2, 3, 4, 5, 6]).unwrap(), [1, 1, 1, 1]);
        assert_eq!(get_sub_layer(&vec![1, 1, 1, 1]).unwrap(), [0, 0, 0,]);
        assert_eq!(get_sub_layer(&vec![0, 0, 0,]), None);
    }

    #[test]
    fn test_get_next_number() {
        assert_eq!(get_next_number(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(get_next_number(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(get_next_number(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_get_previous_number() {
        assert_eq!(get_previous_number(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(get_previous_number(&vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(get_previous_number(&vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
