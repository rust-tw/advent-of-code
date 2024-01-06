mod part1;
mod part2;

use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let ss = input
        .lines()
        .map(|line| line.split(':').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans1 = part1::solve(ss[0][1], ss[1][1]);
    let ans2 = part2::solve(ss[0][1], ss[1][1]);

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");

    Ok(())
}
