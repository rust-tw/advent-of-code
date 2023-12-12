pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|s| {
            let (id, r, g, b) = super::handle_line(s);
            if r <= 12 && g <= 13 && b <= 14 {
                id
            } else {
                0
            }
        })
        .sum()
}
