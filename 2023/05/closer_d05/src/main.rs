mod part1;
mod part2;

use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines: Vec<_> = input.lines().collect();

    let ans1 = part1::solve(&lines);
    let ans2 = part2::solve(&lines);

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");

    Ok(())
}
