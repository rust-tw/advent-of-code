use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: closer_d09.exe <data_file>");
        return Err(Error::new(ErrorKind::InvalidInput, "Not enough argument"));
    }

    let contents = fs::read_to_string(&args[1])?;
    let (ans1, ans2) = solve(&contents.lines().collect::<Vec<_>>());

    println!("ans1 = {ans1}\nans2 = {ans2}");
    Ok(())
}

fn solve(lines: &Vec<&str>) -> (i64, i64) {
    lines
        .iter()
        .map(|line| {
            let history: Vec<_> = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            process_history(&history)
        })
        .reduce(|(acc1, acc2), (n1, n2)| (acc1 + n1, acc2 + n2))
        .unwrap()
}

fn process_history(history: &Vec<i64>) -> (i64, i64) {
    let mut progress = history.clone();
    let mut heads = vec![];
    let mut tails = vec![];
    while progress.iter().any(|&n| n != 0) {
        heads.push(*progress.first().unwrap());
        tails.push(*progress.last().unwrap());
        progress = progress
            .as_slice()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
    }

    (
        tails.into_iter().sum(),
        heads.into_iter().rfold(0, |acc, n| n - acc),
    )
}
