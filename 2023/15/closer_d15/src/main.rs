mod part1;
mod part2;

use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        let file_name = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();
        println!("Usage: {} <data_file>", file_name);
        return Err(Error::new(ErrorKind::InvalidInput, "Not enough argument"));
    }

    let contents = fs::read_to_string(&args[1])?;

    let ans1 = part1::solve(&contents.trim());
    let ans2 = part2::solve(&contents.trim());

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");

    Ok(())
}
