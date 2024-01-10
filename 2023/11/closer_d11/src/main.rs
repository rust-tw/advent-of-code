use closer_d11::solve;
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
    let input = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("{:?}", (solve(&input, 2), solve(&input, 1000000)));

    Ok(())
}
