use std::collections::HashSet;

use anyhow::{Context, Ok, Result};
use itertools::Itertools;
pub fn part1(input: &str) -> Result<usize> {
    let image = Image::parse_with_distance_factor(input, 2)?;
    let sum = sum_galaxies_distance(image);

    Ok(sum)
}

pub fn part2(input: &str) -> Result<usize> {
    let image = Image::parse_with_distance_factor(input, 1000000)?;
    let sum = sum_galaxies_distance(image);

    Ok(sum)
}

fn sum_galaxies_distance(image: Image) -> usize {
    image
        .galaxies
        .iter()
        .enumerate()
        .combinations(2)
        .map(|comb| image.get_distance(comb[0].0, comb[1].0))
        .sum()
}

struct Image {
    galaxies: Vec<(usize, usize)>,
}

impl Image {
    fn parse_with_distance_factor(input: &str, distance_factor: usize) -> Result<Self> {
        let distance_factor = distance_factor - 1;

        let image = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut galaxies = vec![];

        let mut image_iter = image.iter().peekable();

        let all_empty_column = image_iter
            .peek()
            .context("empty image")?
            .iter()
            .enumerate()
            .filter(|(column, _)| {
                (0..image.len())
                    .map(|i| image[i][*column])
                    .all(|c| c == '.')
            })
            .map(|(column, _)| column)
            .collect::<HashSet<usize>>();

        let mut current_y = 0;
        for row in image_iter {
            let mut current_x = 0;
            for (x, c) in row.iter().enumerate() {
                if c == &'#' {
                    galaxies.push((current_x, current_y));
                }

                current_x += 1;

                if all_empty_column.contains(&x){
                    current_x += distance_factor;
                }
            }
            current_y += 1;

            if row.iter().all(|c| *c == '.') {
                current_y += distance_factor;
            }
        }

        Ok(Self { galaxies })
    }

    fn get_distance(&self, source: usize, target: usize) -> usize {
        let (x1, y1) = self.galaxies[source];
        let (x2, y2) = self.galaxies[target];
        x1.max(x2) - x1.min(x2) + y1.max(y2) - y1.min(y2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example");
        assert_eq!(part1(input).unwrap(), 374);
    }

    #[test]
    fn test_get_path() {
        let input = include_str!("../example");
        let image = Image::parse_with_distance_factor(input, 2).unwrap();

        assert_eq!(image.get_distance(4, 8), 9);
        assert_eq!(image.get_distance(0, 6), 15);
    }

    #[test]
    fn test_parse() {
        let input = include_str!("../example");
        let image = Image::parse_with_distance_factor(input, 10).unwrap();

        assert_eq!(sum_galaxies_distance(image), 1030);

        let image = Image::parse_with_distance_factor(input, 100).unwrap();
        assert_eq!(sum_galaxies_distance(image), 8410);
    }
}
