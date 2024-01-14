use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &Vec<Vec<char>>, expand_factor: usize) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let galaxies = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter().enumerate().filter_map(
                move |(x, c)| {
                    if *c == '#' {
                        Some((y, x))
                    } else {
                        None
                    }
                },
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut space_row: HashSet<usize> = HashSet::from_iter(0..height);
    let mut space_col: HashSet<usize> = HashSet::from_iter(0..width);

    galaxies.iter().for_each(|(y, x)| {
        space_row.remove(y);
        space_col.remove(x);
    });

    galaxies
        .into_iter()
        .map(|(y, x)| {
            (
                y + space_row.iter().filter(|&&row| row < y).count() * (expand_factor - 1),
                x + space_col.iter().filter(|&&col| col < x).count() * (expand_factor - 1),
            )
        })
        .combinations(2)
        .map(|galaxies| {
            (galaxies[0].0.abs_diff(galaxies[1].0) + galaxies[0].1.abs_diff(galaxies[1].1)) as u64
        })
        .sum()
}
