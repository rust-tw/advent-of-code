use i_am_closer as my_lib;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines: Vec<_> = input.lines().collect();

    let answers = my_lib::challenge_03(&lines);
    println!("{:?}", answers);

    Ok(())
}
