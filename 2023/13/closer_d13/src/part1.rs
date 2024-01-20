use std::collections::VecDeque;

pub fn solve(patterns: &Vec<Vec<&str>>) -> u64 {
    patterns.iter().map(|pattern| handle_pattern(pattern)).sum()
}

fn handle_pattern(pattern: &Vec<&str>) -> u64 {
    let width = pattern[0].len();
    let height = pattern.len();
    assert!(width < 64);
    assert!(height < 64);

    let mut cols = VecDeque::from_iter(std::iter::repeat(0).take(width));
    let mut rows = VecDeque::from_iter(std::iter::repeat(0).take(height));

    pattern.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            cols[x] <<= 1;
            rows[y] <<= 1;
            if c == '#' {
                cols[x] |= 1;
                rows[y] |= 1;
            }
        })
    });

    evaluate(rows) * 100 + evaluate(cols)
}

fn evaluate(mut src: VecDeque<u64>) -> u64 {
    let mut dest = VecDeque::new();
    while src.len() > 1 {
        dest.push_front(src.pop_front().unwrap());

        let all_equal = src.iter().zip(dest.iter()).all(|(s, d)| *s == *d);
        if all_equal {
            return dest.len() as u64;
        }
    }
    0
}
