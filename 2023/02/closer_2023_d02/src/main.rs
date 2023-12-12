use closer_2023_d02::{part1, part2};
use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!(
        "({},{})",
        part1::solve(&load_contents(1)?),
        part2::solve(&load_contents(2)?)
    );

    Ok(())
}

fn load_contents(sn: usize) -> io::Result<String> {
    let filename = format!("input_part{}.txt", sn);
    let mut f = fs::File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    return Ok(buf);
}
