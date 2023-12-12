pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|s| {
            let (_, r, g, b) = super::handle_line(s);
            r * g * b
        })
        .sum()
}
