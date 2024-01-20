use closer_d17::*;

pub fn solve(input: &str) -> Result<u64, std::io::Error> {
    let v = input.parse::<Walker>()?.walk(1, 3);
    let dist = v.last().unwrap().last().unwrap();

    Ok([dist.horizontal, dist.vertical].into_iter().min().unwrap())
}
