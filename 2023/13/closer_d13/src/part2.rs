use std::collections::VecDeque;

pub fn solve(patterns: &Vec<Vec<&str>>) -> u64 {
    patterns.iter().map(|pattern| handle_pattern(pattern)).sum()
}

fn handle_pattern(pattern: &Vec<&str>) -> u64 {
    let width = pattern[0].len();
    let height = pattern.len();

    let mut cols = VecDeque::from_iter(std::iter::repeat(vec!['.'; height]).take(width));
    let mut rows = VecDeque::from_iter(std::iter::repeat(vec!['.'; width]).take(height));

    pattern.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            cols[x][y] = c;
            rows[y][x] = c;
        })
    });

    evaluate(rows) * 100 + evaluate(cols)
}

fn evaluate(mut src: VecDeque<Vec<char>>) -> u64 {
    let mut dest = VecDeque::new();
    while src.len() > 1 {
        dest.push_front(src.pop_front().unwrap());

        let diff_count = src
            .iter()
            .zip(dest.iter())
            .map(|(s, d)| {
                s.iter()
                    .zip(d.iter())
                    .filter(|(sc, dc)| *sc != *dc)
                    .count()
            })
            .sum::<usize>();

        if diff_count == 1 {
            return dest.len() as u64;
        }
    }
    0
}
