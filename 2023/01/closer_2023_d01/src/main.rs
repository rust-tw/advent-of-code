use closer_2023_d01::{part0, part1};
use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!(
        "({},{})",
        part0::solve(&load_contents(0)?),
        part1::solve(&load_contents(1)?)
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
