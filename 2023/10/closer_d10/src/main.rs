mod part1;
mod part2;

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

    let ans1 = part1::solve(&contents);
    let ans2 = part2::solve(&contents);

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");

    Ok(())
}
