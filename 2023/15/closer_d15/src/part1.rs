use closer_d15::*;

pub fn solve(line: &str) -> u64 {
    line.split(',').map(hashing).sum::<usize>() as u64
}
