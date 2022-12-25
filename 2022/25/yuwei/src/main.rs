fn main() {
    let input = include_str!("../input.txt");
    let num = input
        .lines()
        .map(|s| {
            s.chars().fold(0, |acc, x| {
                acc * 5 + "=-012".chars().position(|i| i == x).unwrap() - 2
            })
        })
        .sum();
    let res = snafu(num);
    println!("{res}");
}

fn snafu(n: usize) -> String {
    if n == 0 {
        "".to_string()
    } else {
        snafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n % 5]
    }
}
