pub fn solve(t_str: &str, d_str: &str) -> u64 {
    let ts = t_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
    let ds = d_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());

    ts.zip(ds)
        .map(|(t, d)| closer_d06::count_single_race(t, d))
        .product()
}
