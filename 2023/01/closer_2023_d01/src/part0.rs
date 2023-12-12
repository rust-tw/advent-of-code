pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|s| {
            let (head, tail) = handle_line(s, (-1, -1));
            (head as u64) * 10 + (tail as u64)
        })
        .sum()
}

fn handle_line(s: &str, (head, tail): (i32, i32)) -> (i32, i32) {
    match s.chars().next() {
        None => (head, tail),
        Some(c) => {
            let (_, rest) = s.split_at(1);
            let new_pair = if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as i32;
                match (head, tail) {
                    (-1, _) => (digit, digit),
                    _ => (head, digit),
                }
            } else {
                (head, tail)
            };
            handle_line(rest, new_pair)
        }
    }
}
