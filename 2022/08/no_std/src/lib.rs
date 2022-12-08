#![no_std]

use itertools::Itertools;

pub fn scan<const N: usize>(grid: [[u8; N]; N]) -> (usize, usize) {
    (0..grid.len()).cartesian_product(0..grid[0].len()).fold(
        (0, 0),
        |(part1, part2), (row, col)| {
            let (visible, score) = solve(&grid, row, col);
            (part1 + visible, part2.max(score))
        },
    )
}

const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn solve<const N: usize>(grid: &[[u8; N]; N], row: usize, col: usize) -> (usize, usize) {
    let tree = grid[row][col];
    let (mut visible, mut score) = (false, 1);

    DIR.iter().for_each(|(x, y)| {
        let (mut r, mut c, mut i, mut v) = (row as isize, col as isize, 0, true);
        while let Some(&next) = grid
            .get((r + x) as usize)
            .and_then(|n| n.get((c + y) as usize))
        {
            i += 1;
            if tree <= next {
                v = false;
                break;
            }
            r += x;
            c += y;
        }
        if v {
            visible = true;
        }
        score *= i;
    });

    (visible as usize, score)
}
