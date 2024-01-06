pub fn solve(t_str: &str, d_str: &str) -> u64 {
    let t = t_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let d = d_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    closer_d06::count_single_race(t, d)
}
