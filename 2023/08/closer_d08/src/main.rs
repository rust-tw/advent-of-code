mod part1;
mod part2;

use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("Usage: closer_d08.exe <part1_data> <part2_data>");
        return Ok(());
    }

    let contents = fs::read_to_string(&args[1])?;
    let ans1 = part1::solve(&contents.lines().collect::<Vec<_>>());

    let contents = fs::read_to_string(&args[2])?;
    let ans2 = part2::solve(&contents.lines().collect::<Vec<_>>());

    println!("ans1 = {ans1}\nans2 = {ans2}");
    Ok(())
}
