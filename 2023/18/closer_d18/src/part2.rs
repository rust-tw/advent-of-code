use closer_d18::*;

pub fn solve(input: &str) -> i64 {
    let (_, _, dirs, steps) = parse_data(input);
    eval_fast(&dirs, &steps)
}
