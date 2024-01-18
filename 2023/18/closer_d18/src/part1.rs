use closer_d18::*;

pub fn solve(input: &str) -> i64 {
    let (dirs, steps, _, _) = parse_data(input);
    // eval_slow(&dirs, &steps)
    eval_fast(&dirs, &steps)
}
