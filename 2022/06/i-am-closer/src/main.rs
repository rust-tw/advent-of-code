use i_am_closer as my_lib;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines: Vec<_> = input.lines().collect();

    let answers = (
        my_lib::challenge_06(lines[0], 4),
        my_lib::challenge_06(lines[0], 14),
    );

    println!("{:?}", answers);

    Ok(())
}
